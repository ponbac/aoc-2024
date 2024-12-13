const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug, Clone)]
struct FileBlock {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone)]
enum DiskEntry {
    FileBlock(FileBlock),
    FreeSpace(usize),
}

struct Disk {
    entries: Vec<DiskEntry>,
    next_id: usize,
}

impl Disk {
    fn new() -> Self {
        Self {
            entries: vec![],
            next_id: 0,
        }
    }

    fn add_file_block(&mut self, size: usize) {
        self.entries.push(DiskEntry::FileBlock(FileBlock {
            id: self.next_id,
            size,
        }));
        self.next_id += 1;
    }

    fn add_free_space(&mut self, size: usize) {
        self.entries.push(DiskEntry::FreeSpace(size));
    }

    fn layout(&self) -> Vec<Option<usize>> {
        let mut result = Vec::new();
        for entry in &self.entries {
            match entry {
                DiskEntry::FileBlock(file) => {
                    for _ in 0..file.size {
                        result.push(Some(file.id));
                    }
                }
                DiskEntry::FreeSpace(size) => {
                    for _ in 0..*size {
                        result.push(None);
                    }
                }
            }
        }
        result
    }

    fn defragmented(&self) -> Vec<Option<usize>> {
        let mut layout = self.layout();

        loop {
            let mut moved = false;

            // Find rightmost file block
            for i in (0..layout.len()).rev() {
                if let Some(id) = layout[i] {
                    // Find leftmost free space before this file block
                    if let Some(free_idx) =
                        (0..i).find(|&j| layout[j].is_none() && (j == 0 || layout[j - 1].is_some()))
                    {
                        layout[free_idx] = Some(id);
                        layout[i] = None;
                        moved = true;
                        break;
                    }
                }
            }

            if !moved {
                break;
            }
        }

        layout
    }

    fn defragmented_keep_files(&self) -> Vec<Option<usize>> {
        let mut layout = self.layout();
        let file_ids: Vec<usize> = (0..self.next_id).rev().collect();

        for &file_id in &file_ids {
            if let Some((current_pos, size)) = self.find_file_bounds(&layout, file_id) {
                if let Some(best_pos) = self.find_best_position(&layout, current_pos, size) {
                    self.move_file_block(&mut layout, current_pos, best_pos, size, file_id);
                }
            }
        }

        layout
    }

    fn find_file_bounds(&self, layout: &[Option<usize>], file_id: usize) -> Option<(usize, usize)> {
        let start_idx = layout.iter().position(|&block| block == Some(file_id))?;

        let size = layout[start_idx..]
            .iter()
            .take_while(|&&block| block == Some(file_id))
            .count();

        Some((start_idx, size))
    }

    fn find_best_position(
        &self,
        layout: &[Option<usize>],
        current_pos: usize,
        size: usize,
    ) -> Option<usize> {
        let mut free_start = None;
        let mut free_size = 0;

        for (i, &block) in layout.iter().enumerate() {
            if i >= current_pos {
                break;
            }

            match block {
                None => {
                    if free_start.is_none() {
                        free_start = Some(i);
                    }
                    free_size += 1;

                    if free_size >= size {
                        return free_start;
                    }
                }
                Some(_) => {
                    free_start = None;
                    free_size = 0;
                }
            }
        }

        None
    }

    fn move_file_block(
        &self,
        layout: &mut [Option<usize>],
        from: usize,
        to: usize,
        size: usize,
        file_id: usize,
    ) {
        (from..from + size).for_each(|i| {
            layout[i] = None;
        });
        for i in 0..size {
            layout[to + i] = Some(file_id);
        }
    }

    fn calculate_checksum(defragged: &[Option<usize>]) -> usize {
        defragged
            .iter()
            .enumerate()
            .filter_map(|(pos, id)| id.map(|id| pos * id))
            .sum()
    }
}

fn main() {
    let input = INPUT
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut disk = Disk::new();
    for (i, &num) in input.iter().enumerate() {
        match i % 2 {
            0 => disk.add_file_block(num),
            1 => disk.add_free_space(num),
            _ => unreachable!(),
        }
    }

    let defragged = disk.defragmented();
    let checksum = Disk::calculate_checksum(&defragged);
    println!("Part 1 Checksum: {}", checksum);

    let defragged_files = disk.defragmented_keep_files();
    let checksum = Disk::calculate_checksum(&defragged_files);
    println!("Part 2 Checksum: {}", checksum);
}
