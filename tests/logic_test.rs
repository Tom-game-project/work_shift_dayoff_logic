#[cfg(test)]
mod logic_test {
    use super::*;
    use shift_calendar::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Deserialize)]
    struct Group {
        staff: Vec<String>,
    }

    type Config = BTreeMap<String, Group>;

    /// char to shifthall 
    fn c2h<'a>(type_char:char, id:usize) -> Option<ShiftHall<'a, Incomplete>> {
        match type_char {
            'a' => Some(ShiftHall::new(0, id)),
            'b' => Some(ShiftHall::new(1, id)),
            _ => None
        }
    }

    fn create_test_data() {
        let week_rule0 = WeekRule([
            DayRule { // Sunday
                shift_morning:vec![],
                shift_afternoon:vec![]
            },
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
            DayRule {
                shift_morning:vec![],
                shift_afternoon:vec![]
            },
        ]);

        let week_rule1 = WeekRule([
            DayRule { // Subday
                shift_morning:vec![],
                shift_afternoon:vec![]
            },
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
            DayRule {
                shift_morning:vec![],
                shift_afternoon:vec![]
            },
        ]);

        let week_rule_table = WeekRuleTable(vec![week_rule0, week_rule1]);

        // Read Staff info from test.toml file
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

        let staff_group_list = StaffGroupList(vec![staff_group_a, staff_group_b]);

        let shift = gen_shift(week_rule_table, &staff_group_list, 25, 5);

        for (week, i) in shift.iter().enumerate() {
            println!("week{} ===========", week);
            for j in &i.0 {
                println!("{:?}", j);
            }
        }
    }

    #[test]
    fn it_works() {
        create_test_data();
    }
}
