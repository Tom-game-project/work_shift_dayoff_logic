use crate::shift_gen::{self, ShiftHoll};

use shift_gen::{
    WeekRuleTable, 
    Incomplete,
    StaffGroupList
};

pub trait CheckRule<'a, Data> {
    type Error;

    fn check_rule(&self, data: &'a Data) -> Result<(), Self::Error>;
}

pub fn verify<'a, Data, R>(
    data: &'a Data,
    rules: &[R],
) -> Result<&'a Data, R::Error>
where
    R: CheckRule<'a, Data>,
{
    match rules.iter().try_for_each(|rule| rule.check_rule(&data)){
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
    GroupIdOutOfRange(&'a ShiftHoll<'a, Incomplete>),
    StaffIdOutOfRange(&'a ShiftHoll<'a, Incomplete>),
    DupHoll
}

pub struct InOfRange();
pub struct NoDupHollPerWeek();

fn fill_check_list<'a>(
    hole: &'a ShiftHoll<'a, Incomplete>,
    check_list:&mut Box<[Box<[Option<()>]>]>) 
-> Result<(), RuleErr<'a>> 
{
    let group_index = hole.group_id;
    let staff_index = hole.id;
    if let Some(a) = check_list.get_mut(group_index) {
        if let Some(b) = a.get_mut(staff_index) {
            *b = Some(());
            Ok(())
        } else {
            return Err(RuleErr { reason: CauseOfRuleErr::StaffIdOutOfRange(&hole) });
        }
    } else {
        return Err(RuleErr { reason: CauseOfRuleErr::GroupIdOutOfRange(&hole) });
    }
}

/// 未使用のstaff indexのチェック。不正なグループid, スタッフidのチェック
impl<'a> CheckRule<'a, (WeekRuleTable<'a, Incomplete>, StaffGroupList)> for  NoDupHollPerWeek {
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
                    CauseOfRuleErr::DupHoll
            })
        }
    }
}
