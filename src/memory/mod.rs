mod page;
mod utils;

use page::Page;
use utils::*;

use rand::prelude::*;

enum Hit {
    Hit(usize, usize),          // hit position, to replace position
    Miss(usize, Option<usize>), //replace position, None/to replace position
}

pub struct Memory {
    pages: Vec<Option<Page>>,
    last_used_position: usize,
    no_of_pages: usize,
    outcome_string: String,
    result: String,
}

impl Memory {
    pub fn new(no_of_pages: usize) -> Self {
        Memory {
            pages: {
                let mut vec = Vec::with_capacity(no_of_pages);
                for _ in 0..no_of_pages {
                    vec.push(None);
                }
                vec
            },
            last_used_position: no_of_pages - 1,
            no_of_pages,
            outcome_string: String::new(),
            result: String::new(),
        }
    }

    // FIFO First In First Out
    pub fn simulate_fifo(&mut self, references: &[u64]) {
        let mut page_faults: u64 = 0;
        let mut page_hits: u64 = 0;
        for &reference in references {
            let mut position: Option<usize> = None;
            let pos = self.next_pos();
            if self.find_reference(reference, &mut position) {
                self.push_step(Hit::Hit(position.unwrap(), pos), reference);
                page_hits += 1;
                continue;
            }
            self.pages[pos] = match self.pages[pos] {
                Some(_) => {
                    self.push_step(Hit::Miss(pos, None), reference);
                    Some(Page::new(reference))
                }
                None => {
                    self.push_step(Hit::Miss(pos, None), reference);
                    Some(Page::new(reference))
                }
            };

            page_faults += 1;
            self.last_used_position = pos;
        }

        self.result = format!(
            "FIFO\npage_hits: {}\npage_misses: {}\n",
            page_hits, page_faults
        );
    }

    // LRU Least Recently Used
    pub fn simulate_lru(&mut self, references: &[u64]) {
        //TODO
        let mut page_faults: u64 = 0;
        let mut page_hits: u64 = 0;

        for &reference in references {
            let mut position: Option<usize> = None;
            let mut pos = self.next_pos();
            if self.find_reference(reference, &mut position) {
                page_hits += 1;
                self.push_step(Hit::Hit(position.unwrap(), pos), reference);
                continue;
            }
            match self.pages[pos] {
                Some(page) => {
                    if page.is_taken() {
                        self.last_used_position = pos;
                        pos = self.next_pos();
                        self.push_step(Hit::Miss(pos, Some(self.last_used_position)), reference);
                        self.pages[pos] = Some(Page::new_taken(reference));
                    } else {
                        self.push_step(Hit::Miss(pos, None), reference);
                        self.pages[pos] = Some(Page::new_taken(reference));
                    }
                }
                None => {
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] = Some(Page::new_taken(reference));
                }
            }

            page_faults += 1;
            self.last_used_position = pos;
        }

        self.result = format!(
            "LRU\npage hits: {}\npage misses: {}\n",
            page_hits, page_faults
        );
    }

    // OPT optimal
    pub fn simulate_opt(&mut self, references: &[u64]) {
        let mut page_faults: u64 = 0;
        let mut page_hits: u64 = 0;

        for (step, &reference) in references.iter().enumerate() {
            let mut position: Option<usize> = None;
            let mut pos = self.next_pos();

            if self.find_reference(reference, &mut position) {
                page_hits += 1;
                self.push_step(Hit::Hit(position.unwrap(), pos), reference);
                continue;
            }

            match self.pages[pos] {
                Some(_) => {
                    pos = self.find_pos_with_longest_length(step, references);
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] = Some(Page::new(reference));
                }
                None => {
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] = Some(Page::new(reference));
                }
            }

            self.last_used_position = pos;
            page_faults += 1;
        }

        self.result = format!(
            "OPT\npage hits: {}\npage misses: {}\n",
            page_hits, page_faults
        );
    }

    // RAND
    pub fn simulate_rand(&mut self, references: &[u64]) {
        //TODO
        let mut page_faults: u64 = 0;
        let mut page_hits: u64 = 0;
        let mut rand = rand::thread_rng();

        for &reference in references {
            let mut position: Option<usize> = None;
            let mut pos = self.next_pos();

            if self.find_reference(reference, &mut position) {
                page_hits += 1;
                self.push_step(Hit::Hit(position.unwrap(), pos), reference);
                continue;
            }

            match self.pages[pos] {
                Some(_) => {
                    pos = rand.gen_range(0, self.no_of_pages);
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] = Some(Page::new(reference));
                }
                None => {
                    self.push_step(Hit::Miss(pos, None), reference);
                    self.pages[pos] = Some(Page::new(reference));
                }
            }

            self.last_used_position = pos;
            page_faults += 1;
        }

        self.result = format!(
            "RAND\npage_hits: {}\npage misses: {}\n",
            page_hits, page_faults
        );
    }

    // aproksymatywny LRU
    pub fn simulate_alru(&mut self, references: &[u64]) {
        //TODO
    }

    pub fn find_pos_with_longest_length(&self, current_step: usize, refs: &[u64]) -> usize {
        let mut vec: Vec<u64> = Vec::new();
        for _ in 0..self.no_of_pages {
            vec.push(u64::max_value());
        }
        for (i, page) in self.pages.iter().enumerate() {
            for (step, &value) in refs[current_step..].iter().enumerate() {
                println!("{:?}", (step, &value));
                if page.unwrap().value() == value {
                    let length = step as u64;
                    if vec[i] > length {
                        vec[i] = length;
                        break;
                    }
                }
            }
        }
        let max = vec.iter().max().unwrap();
        vec.iter().enumerate().find(|n| n.1 == max).unwrap().0
    }

    fn next_pos(&mut self) -> usize {
        let pos = (self.last_used_position + 1) % self.no_of_pages;
        println!("{}", pos);
        pos
    }

    fn find_reference(&mut self, reference: u64, position: &mut Option<usize>) -> bool {
        let mut found = false;

        for (i, option) in self.pages.iter_mut().enumerate() {
            println!("{:?}", (i, &option));
            match option {
                Some(page) => {
                    if page.value() == reference {
                        page.set_taken(true);
                        page.set_ticks(1_u64);
                        found = true;
                        *position = Some(i);
                    } else {
                        if page.ticks() > 0 {
                            page.set_ticks(page.ticks() - 1_u64);
                        } else {
                            page.set_taken(false);
                        }
                    }
                }
                None => {}
            }
        }

        found
    }

    fn push_step(&mut self, hit: Hit, reference: u64) {
        let mut line = format!("[{}] ", reference);
        match hit {
            Hit::Hit(position, to_replace) => {
                line.push_str("#");
                for (i, option) in self.pages.iter().enumerate() {
                    if i == position && position == to_replace {
                        line.push_str(
                            format!(
                                " _|{}|_",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else if i == position {
                        line.push_str(
                            format!(
                                " _{}_",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else if i == to_replace {
                        line.push_str(
                            format!(
                                " |{}|",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else {
                        line.push_str(
                            format!(
                                " {}",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    }
                }
            }
            Hit::Miss(replaced, to_replace_option) => {
                line.push_str("$");
                for (i, option) in self.pages.iter().enumerate() {
                    if let Some(to_replace) = to_replace_option {
                        if i == to_replace {
                            line.push_str(
                                format!(
                                    " |{}|",
                                    match option {
                                        Some(page) => page.value().to_string(),
                                        None => "X".to_owned(),
                                    }
                                )
                                .as_ref(),
                            );
                            continue;
                        }
                    }
                    if i == replaced {
                        line.push_str(
                            format!(
                                " ||{}||",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    } else {
                        line.push_str(
                            format!(
                                " {}",
                                match option {
                                    Some(page) => page.value().to_string(),
                                    None => "X".to_owned(),
                                }
                            )
                            .as_ref(),
                        );
                    }
                }
            }
        }

        line.push('\n');
        self.outcome_string.push_str(line.as_ref());
    }
}

pub fn fifo(pages: usize, references: String) -> (String, String) {
    let mut mem = Memory::new(pages);
    let refs = split_references(references);
    println!("{:?}", &refs);
    mem.simulate_fifo(&refs);

    (mem.result, mem.outcome_string)
}

pub fn lru(pages: usize, references: String) -> (String, String) {
    let mut mem = Memory::new(pages);
    let refs = split_references(references);
    println!("{:?}", &refs);
    mem.simulate_lru(&refs);

    (mem.result, mem.outcome_string)
}

pub fn opt(pages: usize, references: String) -> (String, String) {
    let mut mem = Memory::new(pages);
    let refs = split_references(references);
    println!("{:?}", &refs);
    mem.simulate_opt(&refs);

    (mem.result, mem.outcome_string)
}

pub fn rand(pages: usize, references: String) -> (String, String) {
    let mut mem = Memory::new(pages);
    let refs = split_references(references);
    println!("{:?}", &refs);
    mem.simulate_rand(&refs);

    (mem.result, mem.outcome_string)
}
