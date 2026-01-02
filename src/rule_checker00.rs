use error_combinator::{
    check::{
        Check, CheckOutcome, CheckState, check_noref, check_ref
    },
    cmberr::VecCombine
};

use crate::shift_gen::{
    Incomplete, ShiftHoll, Staff, StaffGroupList, WeekRuleTable 
};

struct IndexUnchecked;
struct IndexChecked;

enum ValidateErr {
    GroupIdOutOfRangeErr(HollIndex),
    StaffIdOutOfRangeErr(HollIndex),
    UnAssignedStaffErr(Vec<StaffIndex>),
}

enum TimeKind{
    AfternoonIndex(usize),
    MorningIndex(usize)
}

struct HollIndex {
    week_rule_index: usize,
    day_rule: usize,
    time_kind: TimeKind
}

struct StaffIndex {
    group_id: usize,
    index: usize
}

fn holes_iter<'a>(
    data: &'a (WeekRuleTable<'a, Incomplete>, StaffGroupList),
) -> impl Iterator<Item = (&'a ShiftHoll<'a, Incomplete>, HollIndex)> + 'a {
    data.0.0.iter().enumerate().flat_map(|(i, week_rule)| {
        week_rule.0.iter().enumerate().flat_map(move |(j, day_rule)| {
            let morning = day_rule
                .shift_morning
                .iter()
                .enumerate()
                .map(move |(k, hole)| {
                    (
                        hole,
                        HollIndex {
                            week_rule_index: i,
                            day_rule: j,
                            time_kind: TimeKind::MorningIndex(k),
                        },
                    )
                });

            let afternoon = day_rule
                .shift_afternoon
                .iter()
                .enumerate()
                .map(move |(k, hole)| {
                    (
                        hole,
                        HollIndex {
                            week_rule_index: i,
                            day_rule: j,
                            time_kind: TimeKind::AfternoonIndex(k),
                        },
                    )
                });

            morning.chain(afternoon)
        })
    })
}

fn fill_check_list<'a>(
    hole: &ShiftHoll<'a, Incomplete>,
    check_list:&[usize],
    holl_index: HollIndex
)
-> Result<(), ValidateErr> 
{
    let group_index = hole.group_id;
    let hole_staff_index = hole.id;

    if let Some(&a) = check_list.get(group_index) {
        if hole_staff_index < a {
            Ok(())
        } else {
            return Err(ValidateErr::GroupIdOutOfRangeErr(holl_index));
        }
    } else {
        return Err(ValidateErr::StaffIdOutOfRangeErr(holl_index));
    }
}

/// 設定されたシフトホールが、スタッフのindexを超えるようなアクセスをしていないかを検査するチェッカー
fn check_index<'a>(data: &(WeekRuleTable<'a, Incomplete>, StaffGroupList)) -> Result<(), ValidateErr> {
    let staff_group_list = &data.1.0;

    let check_list:Box<[usize]> = staff_group_list
        .iter()
        .map(|i| 
            i.len()
        )
        .collect::<Vec<usize>>()
        .into_boxed_slice();

    for (hole, hole_index) in holes_iter(data) {
        fill_check_list(
            &hole,
            &check_list,
            hole_index
        )?;
    }

    Ok(())
}

fn fill_check_unassigned_staff<'a>(
    hole: &ShiftHoll<'a, Incomplete>,
    check_list: &mut Box<[Box<[Option<()>]>]>,
) {
    let group_index = hole.group_id;
    let holl_staff_index = hole.id;

    check_list[group_index][holl_staff_index] = Some(());
}

/// アサインされていないスタッフホールを検出するチェッカー
fn check_unassigned_staff<'a>(data: &(WeekRuleTable<'a, Incomplete>, StaffGroupList)) -> Result<(), ValidateErr> {
    let staff_group_list = &data.1.0;
    let mut check_list = staff_group_list
        .iter()
        .map(|i| 
            (0..i.len())
            .map(|_| None)
            .collect::<Vec<_>>()
            .into_boxed_slice()
        )
        .collect::<Vec<Box<[Option<()>]>>>()
        .into_boxed_slice();

    for (hole, hole_index) in holes_iter(data) {
        fill_check_unassigned_staff(
            &hole,
            &mut check_list,
        );
    }

    let rlist: Vec<StaffIndex> = check_list
        .iter()
        .enumerate()
        .flat_map(|(group_id, staff_group)| {
            staff_group
                .iter()
                .enumerate()
                .filter_map(move |(index, staff)| {
                    staff.is_none().then_some(StaffIndex {
                        group_id,
                        index,
                    })
                })
        })
        .collect();

    if rlist.is_empty() {
        Ok(())
    } else {
        Err(ValidateErr::UnAssignedStaffErr(rlist))
    }
}

fn checker<'a>(data: (WeekRuleTable<'a, Incomplete>, StaffGroupList)) 
-> Result<(WeekRuleTable<'a, Incomplete>, StaffGroupList), Vec<ValidateErr>>
{
    check_noref::<
        (WeekRuleTable<'a, Incomplete>, StaffGroupList),
        IndexUnchecked,
        IndexChecked,
        ValidateErr,
        _
    >(check_index)
    .and::<
    _,
    VecCombine<ValidateErr>
    >(check_noref::<
        (WeekRuleTable<'a, Incomplete>, StaffGroupList),
        IndexChecked,
        IndexChecked,
        ValidateErr,
        _
    >(check_unassigned_staff))
    .check(CheckState::new(data))
    .to_result()
}
