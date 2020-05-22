use super::*;
use crate::msg::Envelope;
use std::sync::{Arc, mpsc::SyncSender};

pub struct Entry {
    pub sender: SyncSender<Arc<Envelope>>,
    pub player: Option<Player>
}

impl Entry {
    fn new(sender: SyncSender<Arc<Envelope>>) -> Self { Self{sender, player: None}}
    fn attach(&mut self, player: Player) -> Option<Player> { self.player.replace(player) }
}

pub struct Store{
    slots: [Option<Entry>; 256],
    set: std::collections::HashSet<(u16, u16)>,
}

impl Store {
    pub fn reserve(&mut self, sender: SyncSender<Arc<Envelope>>) -> Option<u8> {
        for i in 0..self.slots.len() {
            if let None = self.slots[i] {
                self.slots[i] = Some(Entry::new(sender));
                println!("PlayerStore slot#{} reserved", i);
                return Some(i as u8)
            }
        }
        None
    }
    pub fn attach_player(&mut self, player: &mut Player, map_size: (u16, u16)) {
        while player.coord.0 < map_size.0 {
            while player.coord.1 < map_size.1 {
                if self.set.insert(player.coord) {
                    self.slots[player.id as usize].as_mut().unwrap().attach(player.clone());
                    return
                }
                player.coord.1 += 1
            }
            player.coord.0 += 1
        }
        unimplemented!()
    }
    pub fn get_sender(&self, id: u8) -> Option<&SyncSender<Arc<Envelope>>> {
        self.slots[id as usize].as_ref().map(|e| {
            &e.sender
        })
    }
    pub fn send_except(&mut self, except: Option<u8>, env_ptr: Arc<Envelope>) {
        match except {
            None => {
                for e in self.iter() {
                    e.sender.send(env_ptr.clone()).unwrap();
                }
            },
            Some(id) => {
                for i in 0..self.slots.len() {
                    if i == id as usize { continue }
                    if let Some(entry) = &self.slots[i] {
                        entry.sender.send(env_ptr.clone()).unwrap();
                    }
                }
            }
        }
    }
    pub fn iter_mut(&mut self) -> EntryIterMut { EntryIterMut(self.slots.iter_mut()) }
    pub fn iter(&mut self) -> EntryIter { EntryIter(self.slots.iter()) }
    pub fn remove(&mut self, client_id: u8) {
        if let Some(e) = self[client_id].take() {
            if let Some(p) = e.player {
                self.set.remove(&p.coord);
            }
        }
        println!("PlayerStore entry #{} removed", client_id);
    }
    pub fn move_coord(&mut self, current: &(u16, u16), new: &(u16, u16)) -> bool {
        if !self.set.insert(*new) {
            return false
        }
        if self.set.remove(current) {
            true
        } else {
            unimplemented!()
        }
    }
    pub fn relocate(&mut self, move_msg: &MoveMsg, map_size: (u16, u16)) -> bool {
        if let Some(e) = &mut self.slots[move_msg.id as usize] {
            if let Some(player) = &mut e.player {
                if player.life > 0 &&
                move_msg.coord.0 < map_size.0 &&
                move_msg.coord.1 < map_size.1 &&
                coord_diff(&player.coord, &move_msg.coord) == 1 &&
                self.set.insert(move_msg.coord){
                    self.set.remove(&player.coord);
                    player.coord = move_msg.coord;
                    return true;
                }
            }
        }
        false
    } 
}

impl Default for Store {
    fn default() -> Self {
        Self{
            slots: [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,
            None,None,None,None,None],
            set: std::collections::HashSet::default(),
        }
    }
}

impl std::ops::Index<u8> for Store {
    type Output = Option<Entry>;
    fn index(&self, i: u8) -> &Self::Output {
        &self.slots[i as usize]
    }
}

impl std::ops::IndexMut<u8> for Store {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        &mut self.slots[i as usize]
    }
}

pub struct EntryIterMut<'a>(std::slice::IterMut<'a, Option<Entry>>);

impl<'a> Iterator for EntryIterMut<'a> {
    type Item = &'a mut Entry;
    fn next(&mut self) -> Option<Self::Item> {
        for e in &mut self.0 {
            if e.is_none() {
                continue
            }
            return e.as_mut()
        }
        None
    }
}


pub struct EntryIter<'a>(std::slice::Iter<'a, Option<Entry>>);

impl<'a> Iterator for EntryIter<'a> {
    type Item = &'a Entry;
    fn next(&mut self) -> Option<Self::Item> {
        for e in &mut self.0 {
            if e.is_none() {
                continue
            }
            return e.as_ref()
        }
        None
    }
}

fn coord_diff(current: &(u16, u16), new: &(u16, u16)) -> u16 {
    let diff_x = (current.0 as i32 - new.0 as i32).abs();
    let diff_y = (current.1 as i32 - new.1 as i32).abs();
    (diff_x + diff_y) as u16
}

#[cfg(test)]
mod tests {
    use super::coord_diff;

    #[test]
    fn test_coord_diff() {
        let c1 = (5u16, 10u16);
        let c2 = (4u16, 10u16);
        let c3 = (4u16, 10u16);
        let c4 = (4u16, 11u16);
        assert_eq!(coord_diff(&c1, &c2), 1);
        assert_eq!(coord_diff(&c2, &c3), 0);
        assert_eq!(coord_diff(&c1, &c4), 2);
    }
}
