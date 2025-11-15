use std::collections::VecDeque;
use std::collections::BTreeSet;

pub struct Snake {
    occupied_fields: BTreeSet<(u16, u16)>,
    field_queue: VecDeque<(u16, u16)>
}

impl Snake {
    pub fn new(initial_pos: (u16,u16), initial_len: u16) -> Self {
        let mut snake = Self {occupied_fields: BTreeSet::default(), field_queue: VecDeque::default()};
        for _ in 0..initial_len {
            snake.add_segment(initial_pos);
        }
        snake
    }

    pub fn add_segment(&mut self, pos: (u16, u16)) {
        self.field_queue.push_back(pos);
        self.occupied_fields.insert(pos);
    }

    pub fn get_last_segment(&self) -> &(u16, u16) {
        self.field_queue.front().expect("Empty Snake Queue")
    }

    pub fn get_first_segment(&self) -> &(u16, u16) {
        self.field_queue.back().expect("Empty Snake Queue")
    }

    pub fn remove_last_segment(&mut self) {
        let pos = *self.get_last_segment();
        self.occupied_fields.remove(&pos);
        self.field_queue.pop_front();
    }

    pub fn is_inside(&self, pos: &(u16, u16)) -> bool{
        self.occupied_fields.contains(pos)
    }
}