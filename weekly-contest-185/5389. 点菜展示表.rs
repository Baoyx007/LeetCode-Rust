struct Solution;

use leetcode::vec2d;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut map: HashMap<(String, String), i32> = HashMap::new(); // (餐品, 桌号), count)
        let mut tables: HashSet<String> = HashSet::new();
        let mut foods: HashSet<String> = HashSet::new();

        for order in orders {
            if let [_, table, food] = &*order {
                tables.insert(table.to_owned());
                foods.insert(food.to_owned());

                let entry = map.entry((food.to_owned(), table.to_owned())).or_insert(0);
                (*entry) += 1;
            }
        }
        // dbg!(&map);

        // vec![["".to_owned()]]
        let mut ret: Vec<Vec<String>> = Vec::new();

        let mut sort_tables = tables.into_iter().collect::<Vec<String>>();
        sort_tables.sort_by(|a, b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));
        // dbg!(&sort_tables);

        let mut sort_foods = foods.into_iter().collect::<Vec<String>>();
        sort_foods.sort();
        sort_foods.insert(0, "Table".to_owned());
        ret.push(sort_foods);

        // dbg!(&ret);

        for table in sort_tables {
            let mut line = vec![table];

            for food in ret[0].iter().skip(1) {
                // dbg!(&(food.to_string(), line[0].to_string()));
                // dbg!(map
                //     .get(&(food.to_string(), line[0].to_string()))
                //     .unwrap_or(&0)
                //     .to_string());
                line.push(
                    map.get(&(food.to_string(), line[0].to_string()))
                        .unwrap_or(&0)
                        .to_string(),
                );
            }
            ret.push(line);
        }

        // dbg!(&ret);
        ret
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::display_table(vec2d![
            ["David".to_owned(), "3".to_owned(), "Ceviche".to_owned()],
            [
                "Corina".to_owned(),
                "10".to_owned(),
                "Beef Burrito".to_owned()
            ],
            [
                "David".to_owned(),
                "3".to_owned(),
                "Fried Chicken".to_owned()
            ],
            ["Carla".to_owned(), "5".to_owned(), "Water".to_owned()],
            ["Carla".to_owned(), "5".to_owned(), "Ceviche".to_owned()],
            ["Rous".to_owned(), "3".to_owned(), "Ceviche".to_owned()]
        ])
    );
    // assert_eq!("",)
}
