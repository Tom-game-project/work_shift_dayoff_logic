#[cfg(test)]
mod logic_test {
    use shift_calendar::shift_gen::*;
    use shift_calendar::rule_checker::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Deserialize)]
    struct Group {
        staff: Vec<String>,
    }

    type Config = BTreeMap<String, Group>;

    /// char to shiftholl 
    fn c2h<'a>(type_char:char, id:usize) -> Option<ShiftHoll<'a, Incomplete>> {
        match type_char {
            'a' => Some(ShiftHoll::new(0, id)),
            'b' => Some(ShiftHoll::new(1, id)),
            'c' => Some(ShiftHoll::new(2, id)), // for incorrect test case
            _ => None
        }
    }

    // test macro
    macro_rules! h {
        ($id:ident) => {{
            let s = stringify!($id);
            let mut chars = s.chars();
            let c = chars.next().expect("empty ident");
            let n: usize = chars.as_str().parse().expect("invalid number");
            c2h(c, n).unwrap()
        }};
    }
    macro_rules! day {
        (
            m[$($m:ident),* $(,)?],
            a[$($a:ident),* $(,)?]
        ) => {
            DayRule {
                shift_morning: vec![$(h!($m)),*],
                shift_afternoon: vec![$(h!($a)),*],
            }
        };
    }
    macro_rules! week_rule {
        (
            $(
                $day:ident :
                m[$($m:ident),* $(,)?],
                a[$($a:ident),* $(,)?]
            ),* $(,)?
        ) => {
            WeekRule([
                $(
                    day!(m[$($m),*], a[$($a),*])
                ),*
            ])
        };
    }

    fn treat_error(e: RuleErr) {
        match e.reason {
            CauseOfRuleErr::DupHoll => {
                println!("DupHoll!")
            }
            CauseOfRuleErr::GroupIdOutOfRange(hole) => {
                println!("group id {} GroupIdOutOfRange! ", hole.group_id)
            }
            CauseOfRuleErr::StaffIdOutOfRange(hole) => {
                println!("id {} StaffIdOutOfRange!", hole.id);
            }
        }
    }

    fn create_test_data() {
        let week_rule0 = week_rule![
            mon: m[a0, b0],  a[b1],
            tue: m[],        a[a1],
            wed: m[],        a[],
            thu: m[b4],      a[],
            fri: m[b5, b2],  a[a3, b3, a2],
            sat: m[],        a[],
            sun: m[],        a[],
        ];
        let week_rule1 = week_rule![
            mon: m[a2, b3],  a[b2],
            tue: m[],        a[b4],
            wed: m[],        a[],
            thu: m[a1],      a[],
            fri: m[b1, b3],  a[b5, a0, b0],
            sat: m[],        a[],
            sun: m[],        a[],
        ];

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

        match verify(
            &(week_rule_table, staff_group_list),
            &[NoDupHollPerWeek()]
        ) {
            Ok((week_rule_table, staff_group_list)) => {
                let shift = gen_shift(&week_rule_table, &staff_group_list, 25, 5);
                for (week, i) in shift.iter().enumerate() {
                    println!("week{} ===========", week);
                    for j in &i.0 {
                        println!("{:?}", j);
                    }
                }
            }
            Err(e) => {
                treat_error(e);
            }

        }
    }

    #[test]
    fn it_works00() {
        create_test_data();
    }

    #[test]
    fn it_works01() {
        // This data has index issues.
        let week_rule0 = week_rule![
            mon: m[a0, b0],  a[b1],
            tue: m[],        a[a1], // <- Error!
            wed: m[],        a[],
            thu: m[a1],      a[],
            fri: m[b5, b2],  a[a3, b3, a2],
            sat: m[],        a[],
            sun: m[],        a[],
        ];
        let week_rule1 = week_rule![
            mon: m[a2, b3],  a[b2],
            tue: m[],        a[b4],
            wed: m[],        a[],
            thu: m[b4],      a[],
            fri: m[b1, b3],  a[b5, a0, b0],
            sat: m[],        a[],
            sun: m[],        a[],
        ];

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

        match verify(
            &(week_rule_table, staff_group_list),
            &[NoDupHollPerWeek()]
        ) {
            Ok((week_rule_table, staff_group_list)) => {
                let shift = gen_shift(&week_rule_table, &staff_group_list, 25, 5);
                for (week, i) in shift.iter().enumerate() {
                    println!("week{} ===========", week);
                    for j in &i.0 {
                        println!("{:?}", j);
                    }
                }
            }
            Err(e) => {
                treat_error(e);
            }

        }
    }

    fn it_works02() {
        let week_rule0 = week_rule![
            mon: m[a0, b0], a[b1],
            tue: m[],        a[a1],
            wed: m[],        a[],
            thu: m[b4],      a[],
            fri: m[b5, b2],  a[a3, b3, a2],
            sat: m[],        a[],
            sun: m[],        a[],
        ];
    }
}
