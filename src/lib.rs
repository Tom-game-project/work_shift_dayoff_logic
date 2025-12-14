use std::marker::PhantomData;

/// State
#[derive(Clone)]
pub struct Incomplete;
/// State
pub struct Ready;

/// Rule Data
///
/// abstruct shift hall
#[derive(Clone)]
pub struct ShiftHall<'a, State> {
    group_id: usize,
    id: usize,
    staff: Option<&'a Staff>,
    _state: PhantomData<State>
}

pub struct StaffGroupList(
    pub Vec<StaffGroup>
);

trait FillHoll<'a> {
    type Output;

    fn set_self_from_staff_list(
        self,
        staff_group_list: &'a StaffGroupList,
        delta: usize
    ) -> Self::Output;
}

impl<'a> ShiftHall<'a, Incomplete> {
    pub fn new(group_id: usize, id: usize) -> Self{
        Self { group_id , id, staff: None, _state: PhantomData }
    }
}

impl<'a> FillHoll<'a> for ShiftHall<'a, Incomplete> {
    type Output = ShiftHall<'a, Ready>;

    fn set_self_from_staff_list(
        self,
        staff_group_list: &'a StaffGroupList,
        delta: usize
    ) -> Self::Output {
        let staff_group = &staff_group_list.0[self.group_id /*group id must be less than staff_group_list length*/];
        let staff = staff_group.pickup_staff((delta + self.id) % staff_group.staff_list.len());
        ShiftHall {
            group_id: self.group_id,
            id: self.id, 
            staff: Some(staff),
            _state: PhantomData 
        }
    }
}

impl<'a> ShiftHall<'a, Ready> {
    fn gen_decided(&self) -> Option<&'a Staff> {
        self.staff
    }
}

/// Rule Data
///
/// shift a day
#[derive(Clone)]
pub struct DayRule<'a, State> {
    pub shift_morning: Vec<ShiftHall<'a, State>>,
    pub shift_afternoon: Vec<ShiftHall<'a, State>>,
}

impl<'a> FillHoll<'a> for  DayRule<'a, Incomplete> {
    type Output = DayRule<'a, Ready>;

    fn set_self_from_staff_list(
        self,
        staff_group_list: &'a StaffGroupList,
        delta: usize)
        -> Self::Output
    {
        let mut shift_morning: Vec<ShiftHall<'_, Ready>> = vec![];
        let mut shift_afternoon: Vec<ShiftHall<'_, Ready>> = vec![];
        for i in self.shift_morning {
            shift_morning.push(
                i.set_self_from_staff_list(staff_group_list, delta)
            );
        }
        for i in self.shift_afternoon {
            shift_afternoon.push(
                i.set_self_from_staff_list(staff_group_list, delta)
            );
        }
        DayRule { shift_morning, shift_afternoon }
    }
}

#[derive(Debug, Clone)]
pub struct DayDecidedShift<'a> {
    pub shift_morning: Vec<Option<&'a Staff>>,
    pub shift_afternoon: Vec<Option<&'a Staff>>,
}

impl<'a> DayRule<'a, Ready> {
    fn gen_decided(&self) -> DayDecidedShift<'a> {
        let shift_morning: Vec<Option<&'a Staff>>  = self
            .shift_morning
            .iter()
            .map(|hole| hole.gen_decided())
            .collect();
        let shift_afternoon: Vec<Option<&'a Staff>> = self
            .shift_afternoon
            .iter()
            .map(|hole| hole.gen_decided())
            .collect();
        DayDecidedShift { shift_morning, shift_afternoon }
    }
}

/// Rule Data
#[derive(Clone)]
pub struct WeekRule<'a, State> (
    pub [DayRule<'a, State>; 7]
);

impl<'a> FillHoll<'a> for WeekRule<'a, Incomplete> {
    type Output = WeekRule<'a, Ready>;

    fn set_self_from_staff_list(self, staff_group_list: &'a StaffGroupList, delta: usize) -> Self::Output {
        WeekRule(
            self
                .0
                .map(|a| a.set_self_from_staff_list(staff_group_list, delta))
        )
    }
}

pub struct WeekDecidedShift<'a>(
    pub [DayDecidedShift<'a>; 7]
);

impl<'a> WeekRule<'a, Ready> {
    fn gen_decided(&self) -> WeekDecidedShift<'a> {
        WeekDecidedShift(
            std::array::from_fn(|i| self.0[i].gen_decided())
        )
    }
}

#[derive(Clone)]
pub struct WeekRuleTable<'a, State>(
    pub Vec<WeekRule<'a, State>>
);

pub fn gen_shift<'a>(
    week_rule_table: WeekRuleTable<'a, Incomplete>,
    staff_group_list: &'a StaffGroupList,
    week_delta: usize,
    week_gen_range:usize) -> Box<[WeekDecidedShift<'a>]>
{
    let cycle = week_rule_table.0.len();

    (0..week_gen_range)
        .map(|i| {
            week_rule_table.0[(week_delta + i) % cycle].clone() // the rule that apply to
        })
        .enumerate()
        .map(|(i, j)| 
            j
                .set_self_from_staff_list(staff_group_list, 
                    (week_delta + i) / cycle // the number that applied rules
                )
        )
        .map(|i|
            i.gen_decided()
        )
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

// ========= names ===========

/// Staff Info
#[derive(Debug)]
pub struct Staff {
    pub name: String,
    id: usize
}

impl Staff {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), id: 0 }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

/// Staff Info
pub struct StaffGroup {
    staff_list: Vec<Staff>,
}

impl StaffGroup {
    pub fn new() -> Self {
        Self { staff_list: vec![] }
    }

    pub fn add_staff(&mut self, name:&str) {
        self.staff_list.push(
            Staff { name: name.to_string(), id: self.staff_list.len() });
    }
}

impl StaffGroup{
    /// assign staff
    fn pickup_staff<'a>(&'a self, index:usize) -> &'a Staff {
        &self.staff_list[index]
    }
}

