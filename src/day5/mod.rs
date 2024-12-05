use std::collections::HashMap;
use std::str::FromStr;

pub fn part1() {
    let mut input = std::str::from_utf8(include_bytes!("../../res/day5/part1")).unwrap();
    println!("{}", ordered_pages(&mut input));
}

pub fn part2() {
    let mut input = std::str::from_utf8(include_bytes!("../../res/day5/part1")).unwrap();
    println!("{}", reordered_pages(&mut input));
}

fn ordered_pages(input: &str) -> u32 {
    let (ordering_rules, page_updates) = input.split_once("\n\n").unwrap();
    let page_orderings = PageOrderings::from_str(ordering_rules).unwrap();
    let page_updates = PageUpdates::from_str(page_updates).unwrap();

    page_updates.filter_pages(&page_orderings)
}

fn reordered_pages(input: &str) -> u32 {
    let (ordering_rules, page_updates) = input.split_once("\n\n").unwrap();
    let page_orderings = PageOrderings::from_str(ordering_rules).unwrap();
    let mut page_updates = PageUpdates::from_str(page_updates).unwrap();
    
    let mut broken_pages = page_updates.broken_pages(&page_orderings);
    broken_pages.iter_mut().for_each(|page| page.fix_page(&page_orderings));
    let fixed_pages = broken_pages;
    fixed_pages.into_iter().map(|page| page.middle_item()).sum()
}

struct PageOrderings {
    orderings: HashMap<u32, Vec<u32>>,
}

impl FromStr for PageOrderings {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut orderings = HashMap::new();
        input.lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(lhs, rhs)| (lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap()))
            .for_each(|(lhs, rhs)| {
                orderings.entry(lhs).or_insert(Vec::new()).push(rhs);
            });

        Ok(PageOrderings { orderings })
    }
}

struct PageUpdates {
    pages: Vec<Page>,
}

#[derive(Debug)]
struct Page {
    values: Vec<u32>
}

impl FromStr for PageUpdates {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pages = input.lines()
            .map(|line| Page::from_str(line).unwrap())
            .collect::<Vec<_>>();
        Ok(PageUpdates { pages })
    }
}

impl FromStr for Page {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values = input.split(',').map(|item| item.parse::<u32>().unwrap()).collect::<Vec<_>>();
        Ok(Page { values })
    }
}

impl PageUpdates {
    fn filter_pages(&self, orderings: &PageOrderings) -> u32 {
        self.pages.iter().filter(|page| page.validate_page(orderings)).map(Page::middle_item).sum()
    }
    
    fn broken_pages(&mut self, ordering: &PageOrderings) -> Vec<&mut Page> {
        self.pages.iter_mut().filter(|page| !page.validate_page(ordering)).collect::<Vec<_>>()
    }
}

impl Page {
    fn validate_page(&self, orderings: &PageOrderings) -> bool {
        self.values.iter().enumerate().all(|(index, item)| orderings.check_item(*item ,index, &self.values).is_none())
    }

    fn middle_item(&self) -> u32 {
        let middle = self.values.len() / 2;
        self.values[middle]
    }
    
    fn fix_page(&mut self, orderings: &PageOrderings) {
        let misplaced_value = self.values
            .iter()
            .enumerate()
            .find_map(|(index, item)| if let Some(broken_position) = orderings.check_item(*item, index, &self.values) {
                Some((broken_position, index))
            } else {
                None
            });
        
        match misplaced_value {
            Some((broken_pos, index)) => { 
                self.values.swap(index, broken_pos);
                self.fix_page(orderings);
            },
            None => ()
        }
    }
}

impl PageOrderings {
    fn check_item(&self, item: u32, index: usize, values: &[u32]) -> Option<usize> {
        let values = &values[0..index];
        match self.orderings.get(&item) {
            Some(constraints) => values.iter().enumerate().find_map(|(index, value)| if constraints.contains(value) {
                Some(index)
            } else {
                None
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(143, ordered_pages(input));
    }

    #[test]
    fn part2() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(123, reordered_pages(input));
    }
}