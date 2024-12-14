const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug, Clone)]
struct FileBlock {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone)]
enum DiskEntry {
    File(FileBlock),
    Free(usize),
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

    fn add_entry(&mut self, size: usize, is_file: bool) {
        let entry = if is_file {
            DiskEntry::File(FileBlock {
                id: self.next_id,
                size,
            })
        } else {
            DiskEntry::Free(size)
        };

        if is_file {
            self.next_id += 1;
        }
        self.entries.push(entry);
    }

    fn layout(&self) -> Vec<Option<usize>> {
        self.entries
            .iter()
            .flat_map(|entry| match entry {
                DiskEntry::File(file) => vec![Some(file.id); file.size],
                DiskEntry::Free(size) => vec![None; *size],
            })
            .collect()
    }

    fn defragmented(&self) -> Vec<Option<usize>> {
        let mut layout = self.layout();

        let mut changed = true;
        while changed {
            changed = false;
            for i in (0..layout.len()).rev() {
                if let Some(id) = layout[i] {
                    if let Some(free_idx) =
                        (0..i).find(|&j| layout[j].is_none() && (j == 0 || layout[j - 1].is_some()))
                    {
                        layout[free_idx] = Some(id);
                        layout[i] = None;
                        changed = true;
                        break;
                    }
                }
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
        let start = layout.iter().position(|&block| block == Some(file_id))?;
        let size = layout[start..]
            .iter()
            .take_while(|&&block| block == Some(file_id))
            .count();
        Some((start, size))
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
        layout[from..from + size].fill(None);
        layout[to..to + size].fill(Some(file_id));
    }
}

fn calculate_checksum(layout: &[Option<usize>]) -> usize {
    layout
        .iter()
        .enumerate()
        .filter_map(|(pos, &id)| id.map(|id| pos * id))
        .sum()
}

fn main() {
    let disk = INPUT
        .trim()
        .chars()
        .enumerate()
        .fold(Disk::new(), |mut disk, (i, c)| {
            disk.add_entry(c.to_digit(10).unwrap() as usize, i % 2 == 0);
            disk
        });

    println!(
        "Part 1 Checksum: {}",
        calculate_checksum(&disk.defragmented())
    );
    println!(
        "Part 2 Checksum: {}",
        calculate_checksum(&disk.defragmented_keep_files())
    );
}
