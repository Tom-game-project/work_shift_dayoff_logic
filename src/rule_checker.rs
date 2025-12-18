use crate::shift_gen::{self, ShiftHoll};

use shift_gen::{
    WeekRuleTable, 
    Incomplete,
    StaffGroupList,
    Staff
};

pub trait CheckRule<'a, Data> {
    type Error;

    fn check_rule(&self, data: &'a Data) -> Result<(), Self::Error>;
}

pub fn verify<'a, Data, E>(
    data: &'a Data,
    rules: &[&dyn CheckRule<'a, Data, Error = E>],
) -> Result<&'a Data, E>
{
    match rules
        .iter()
        .try_for_each(|rule| 
            rule.check_rule(&data)){
        Ok(()) => {
            Ok(&data)
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub struct RuleErr<'a> {
    pub reason: CauseOfRuleErr<'a> 
}

pub enum CauseOfRuleErr<'a> {
    GroupIdOutOfRangeErr(&'a ShiftHoll<'a, Incomplete>),
    StaffIdOutOfRangeErr(&'a ShiftHoll<'a, Incomplete>),
    AmPmErr(), // 午後午前の指定回数を超えている場合
    DupHollErr
}

pub struct AmPmErr {
    err_case: Box<[Box<[AmPmCounter]>]>
}

pub struct BasicChecker();
pub struct AmPmChecker {
    morning_count: usize,
    afternoon_count: usize,
}

impl AmPmChecker {
    pub fn new(morning_count: usize, afternoon_count: usize) -> Self {
        Self { morning_count, afternoon_count }
    }
}

fn fill_check_list<'a>(
    hole: &'a ShiftHoll<'a, Incomplete>,
    check_list:&mut Box<[Box<[Option<()>]>]>
)
-> Result<(), RuleErr<'a>> 
{
    let group_index = hole.group_id;
    let staff_index = hole.id;
    if let Some(a) = check_list.get_mut(group_index) {
        if let Some(b) = a.get_mut(staff_index) {
            *b = Some(());
            Ok(())
        } else {
            return Err(RuleErr { reason: CauseOfRuleErr::StaffIdOutOfRangeErr(&hole) });
        }
    } else {
        return Err(RuleErr { reason: CauseOfRuleErr::GroupIdOutOfRangeErr(&hole) });
    }
}

/// 未使用のstaff indexのチェック。不正なグループid, スタッフidのチェック
impl<'a> CheckRule<'a, (WeekRuleTable<'a, Incomplete>, StaffGroupList)> for BasicChecker {
    type Error = RuleErr<'a>;

    fn check_rule(&self, data: &'a (WeekRuleTable<'a, Incomplete>, StaffGroupList)) -> Result<(), Self::Error> {
        let staff_group_list = &data.1.0;
        let mut check_list:Box<[Box<[Option<()>]>]> = staff_group_list
            .iter()
            .map(|i| 
                (0..i.len())
                .map(|_| None)
                .collect::<Vec<_>>()
                .into_boxed_slice()
            )
            .collect::<Vec<Box<[Option<()>]>>>()
            .into_boxed_slice();

        for week_rule in &data.0.0 {
            for day_rule in &week_rule.0 {
                for hole in &day_rule.shift_morning {
                    fill_check_list(&hole, &mut check_list)?;
                }
                for hole in &day_rule.shift_afternoon {
                    fill_check_list(&hole, &mut check_list)?;
                }
            }
        }

        if check_list.iter().all(|a| a.iter().all(|b| b.is_some())) {
            Ok(())
        } else {
            Err(RuleErr { 
                reason: 
                    CauseOfRuleErr::DupHollErr
            })
        }
    }
}

struct AmPmCounter{
    marker: bool,
    morning_count: usize,
    afternoon_count: usize,
}

impl AmPmCounter {
    fn set_marker(&mut self, mark:bool) {
        self.marker = mark;
    }
}

fn count_staff_list<'a, F>(
    hole: &'a ShiftHoll<'a, Incomplete>,
    check_list:&mut Box<[Box<[AmPmCounter]>]>,
    f: F
) -> Result<(), RuleErr<'a>>
where F: Fn(&mut AmPmCounter)
{
    let group_index = hole.group_id;
    let staff_index = hole.id;
    if let Some(a) = check_list.get_mut(group_index) {
        if let Some(b) = a.get_mut(staff_index) {
            f(b);
            Ok(())
        } else {
            return Err(RuleErr { reason: CauseOfRuleErr::StaffIdOutOfRangeErr(&hole) });
        }
    } else {
        return Err(RuleErr { reason: CauseOfRuleErr::GroupIdOutOfRangeErr(&hole) });
    }
}

impl<'a> CheckRule<'a, (WeekRuleTable<'a, Incomplete>, StaffGroupList)> for AmPmChecker {
    type Error = RuleErr<'a>;

    fn check_rule(&self, data: &'a (WeekRuleTable<'a, Incomplete>, StaffGroupList)) -> Result<(), Self::Error> {

        // like staff group_list data form
        let mut staff_group_list_counter:Box<[Box<[AmPmCounter]>]> = data
            .1
            .0
            .iter()
            .map(|a|
                (0..a.len())
                .map(|i| 
                    AmPmCounter { 
                        morning_count: 0, 
                        afternoon_count: 0, 
                        marker:false 
                    })
                .collect::<Vec<_>>()
                .into_boxed_slice()
            )
            .collect::<Vec<_>>()
            .into_boxed_slice();

        for week_rule in &data.0.0 {
            for day_rule in &week_rule.0 {
                for hole in &day_rule.shift_morning {
                    count_staff_list(&hole, &mut staff_group_list_counter, |a| {
                        a.morning_count += 1;
                    })?;
                }
                for hole in &day_rule.shift_morning {
                    count_staff_list(&hole, &mut staff_group_list_counter, |a| {
                        a.afternoon_count += 1;
                    })?;
                }
            }
        }

        let a:Box<[Box<[&mut AmPmCounter]>]> = staff_group_list_counter
            .iter_mut()
            .map(|ampm_arr| 
                ampm_arr
                .iter_mut()
                .map(|i| {
                    i.set_marker(
                        i.morning_count != self.morning_count || i.afternoon_count != self.afternoon_count);
                    i
                }
                )
                .collect::<Vec<_>>()
                .into_boxed_slice()
            )
            .collect::<Vec<_>>()
            .into_boxed_slice();

        if a
            .iter()
            .all(|i| 
                i
                .iter()
                .all(|j| 
                    j.marker)) 
        {
            Ok(())
        } else {
            Err(
                RuleErr { reason: 
                    CauseOfRuleErr::AmPmErr()
                }
            )
        }
    }
}
