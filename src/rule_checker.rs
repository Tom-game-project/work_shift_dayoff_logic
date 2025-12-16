use crate::shift_gen;

use shift_gen::{
    WeekRuleTable, 
    Incomplete,
    StaffGroupList
};

pub trait CheckRule<Data> {
    type Error;

    fn check_rule(&self, data: &Data) -> Result<(), Self::Error>;
}

pub struct InOfRange(
);

pub struct RuleErr {
    cor: CauseOfRuleErr 
}

pub enum CauseOfRuleErr {
    GroupIdOutOfRange,
}

pub fn verify<'a, Data, R>(
    data: &'a Data,
    rules: &[R],
) -> Result<&'a Data, R::Error>
where
    R: CheckRule<Data>,
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

impl<'a> CheckRule<(&'a WeekRuleTable<'a, Incomplete>, &'a StaffGroupList)> for InOfRange {
    type Error = RuleErr;
    fn check_rule(&self, data: &(&'a WeekRuleTable<'a, Incomplete>, &'a StaffGroupList)) 
        -> Result<(), Self::Error> {
        for week_rule in &data.0.0 {
            for day_rule in &week_rule.0 {
                for i in &day_rule.shift_morning {
                    if i.group_id < data.1.0.len() {
                    }
                    else {
                        return Err(RuleErr { cor: CauseOfRuleErr::GroupIdOutOfRange });
                    }
                }
                for i in &day_rule.shift_afternoon {
                    if i.group_id < data.1.0.len() {
                    }
                    else {
                        return Err(RuleErr { cor: CauseOfRuleErr::GroupIdOutOfRange });
                    }
                }
            }
        }
        Ok(())
    }
}


