use std::marker::PhantomData;

/// State
struct Incomplete;
/// State
struct Ready;

/// Rule Data
///
/// abstruct shift hall
struct ShiftHall<'a, State> {
    group_id: usize,
    id: usize,
    staff: Option<&'a Staff>,
    _state: PhantomData<State>
}

struct StaffGroupList<'a>(&'a[StaffGroup; 2]);

impl<'a> ShiftHall<'a, Incomplete> {
    fn set_self_from_staff_list(self, staff_group_list: &'a StaffGroupList, delta: usize) -> ShiftHall<'a, Ready> {
        let staff_group = &staff_group_list.0[self.group_id];
        let staff = staff_group.pickup_staff((delta + self.id) % staff_group.staff_list.len());
        ShiftHall {
            group_id: self.group_id,
            id: self.id, 
            staff: Some(staff),
            _state: PhantomData 
        }
    }
}

/// Rule Data
///
/// shift a day
struct DayRule<'a, State> {
    shift_morning: Vec<ShiftHall<'a, State>>,
    shift_afternoon: Vec<ShiftHall<'a, State>>,
}

impl<'a> DayRule<'a, Incomplete> {
    fn set_self_from_staff_list(self, staff_group_list: &'a StaffGroupList, delta: usize) -> DayRule<'a, Ready> {
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

/// Rule Data
struct WeekRule<'a, State> (
    [DayRule<'a, State>; 5]
);

impl<'a> WeekRule<'a, Incomplete> {
    fn set_self_from_staff_list(self, staff_group_list: &'a StaffGroupList, delta: usize) -> WeekRule<'a, Ready> {
        WeekRule(
            self
                .0
                .map(|a| a.set_self_from_staff_list(staff_group_list, delta))
        )
    }
}

// ========= names ===========

/// Staff Info
struct Staff {
    name: String,
    id: usize
}

impl Staff {
    fn set_id(&mut self, id:usize) {
        self.id = id;
    }
}

/// Staff Info
struct StaffGroup {
    staff_list: Vec<Staff>,
}

impl StaffGroup {
    fn new() -> Self {
        Self { staff_list: vec![] }
    }

    fn add_staff(&mut self, name:&str) {
        self.staff_list.push(
            Staff { name: name.to_string(), id: self.staff_list.len() });
    }
}

impl StaffGroup{
    fn pickup_staff<'a>(&'a self, index:usize) -> &'a Staff {
        &self.staff_list[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Deserialize)]
    struct Group {
        staff: Vec<String>,
    }

    type Config = BTreeMap<String, Group>;

    /// char to hall 
    fn c2h<'a>(type_char:char, id:usize) -> Option<ShiftHall<'a, Incomplete>> {
        match type_char {
            'a' => Some(ShiftHall {group_id: 0, id: id, staff: None, _state: PhantomData}),
            'b' => Some(ShiftHall {group_id: 1,id: id, staff: None, _state: PhantomData}),
            _ => None
        }
    }

    fn create_staff(name: &str) -> Staff {
        Staff { name: name.to_string(), id: 0 }
    }

    fn create_test_data() {
        let week_rule0 = WeekRule([
            DayRule {
                shift_morning:vec![c2h('a', 0).unwrap(), c2h('b', 0).unwrap()],
                shift_afternoon:vec![c2h('b', 1).unwrap()]
            },
            DayRule {
                shift_morning:vec![],
                shift_afternoon:vec![c2h('a', 1).unwrap()]
            },
            DayRule {
                shift_morning:vec![],
                shift_afternoon:vec![]
            },
            DayRule {
                shift_morning:vec![c2h('b', 4).unwrap()],
                shift_afternoon:vec![]
            },
            DayRule {
                shift_morning:vec![c2h('b', 5).unwrap(), c2h('b', 2).unwrap()],
                shift_afternoon:vec![c2h('a', 3).unwrap(), c2h('b', 3).unwrap(), c2h('a', 2).unwrap()]
            },
        ]);

        let week_rule1 = WeekRule([
            DayRule {
                shift_morning:vec![c2h('a', 2).unwrap(), c2h('b', 3).unwrap()],
                shift_afternoon:vec![c2h('b', 2).unwrap()]
            },
            DayRule {
                shift_morning:vec![],
                shift_afternoon:vec![c2h('b', 4).unwrap()]
            },
            DayRule {
                shift_morning:vec![],
                shift_afternoon:vec![]
            },
            DayRule {
                shift_morning:vec![c2h('a', 1).unwrap()],
                shift_afternoon:vec![]
            },
            DayRule {
                shift_morning:vec![c2h('b', 1).unwrap(), c2h('b', 3).unwrap()],
                shift_afternoon:vec![c2h('b', 5).unwrap(), c2h('a', 0).unwrap(), c2h('b', 0).unwrap()]
            },
        ]);

        let s = std::fs::read_to_string("test.toml").unwrap();
        let groups: Config = toml::from_str(&s).unwrap();
        let mut staff_group_a = StaffGroup::new();
        for name in &groups["A"].staff {
            staff_group_a.add_staff(name);
        }
        let mut staff_group_b = StaffGroup::new();
        for name in &groups["B"].staff {
            staff_group_b.add_staff(name);
        }

        let staff_group_list = StaffGroupList(&[staff_group_a, staff_group_b]);

    }

    #[test]
    fn it_works() {

    }
}
