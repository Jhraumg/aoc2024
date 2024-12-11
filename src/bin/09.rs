advent_of_code::solution!(9);

type Block = Option<usize>;

fn read_disk(input: &str) -> Vec<Block> {
    let mut disk: Vec<Block> = Vec::with_capacity(input.as_bytes().len());
    let mut fileid = 0;
    let mut describing_file = true;
    for val in input.trim().chars().map(|c| c.to_digit(10).unwrap()) {
        if val == 0 && describing_file {
            panic!("describing empty file {fileid} ?");
        }
        for _ in 0..val as usize {
            disk.push(Some(fileid).filter(|_| describing_file));
        }
        if describing_file {
            fileid += 1;
        }
        describing_file = !describing_file;
    }
    disk
}

#[allow(dead_code)]
fn print_disk(disk: &[Block]) {
    for b in disk {
        print!("{}", b.map_or(".".to_string(), |b| (b % 10).to_string()));
    }
    println!();
}
pub fn part_one(input: &str) -> Option<usize> {
    let disk = read_disk(input);

    let mut blocks_it = disk.iter();
    let mut rev_fblocks_it = disk.iter().enumerate().filter(|(_, b)| b.is_some()).rev();

    let mut defragmented_d: Vec<Block> = Vec::with_capacity(disk.len());
    let mut forward_id = 0;
    let mut rev_id = disk.len() - 1;
    while forward_id < rev_id {
        forward_id += 1;
        if let Some(Some(fileid)) = blocks_it.next() {
            defragmented_d.push(Some(*fileid));
        } else if let Some((j, b)) = rev_fblocks_it.next() {
            rev_id = j;
            if forward_id >= rev_id {
                break;
            }
            defragmented_d.push(Some(b.unwrap()));
        }
    }

    assert_eq!(
        disk.iter().filter(|b| b.is_some()).count(),
        defragmented_d.iter().filter(|b| b.is_some()).count()
    );

    Some(
        defragmented_d
            .iter()
            .enumerate()
            .map(|(i, b)| b.unwrap_or(0) * i)
            .sum(),
    )
}

#[derive(Debug, Copy, Clone, Default)]
struct Span {
    file: Option<usize>,
    len: usize,
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk: Vec<Span> = Vec::with_capacity(input.as_bytes().len());
    let mut fileid = 0;
    let mut describing_file = true;
    for val in input.trim().chars().map(|c| c.to_digit(10).unwrap()) {
        if val == 0 && describing_file {
            panic!("describing empty file {fileid} ?");
        }
        let span = Span {
            file: Some(fileid).filter(|_| describing_file),
            len: val as usize,
        };
        disk.push(span);
        if describing_file {
            fileid += 1;
        }
        describing_file = !describing_file;
    }

    let last_file = disk.last().unwrap().file.unwrap();
    'file: for fileid in (1..last_file + 1).rev() {
        let mut free_span: Span;
        let free_place;
        let new_free_len;
        let moved_file_place;
        {
            let (i, moved_span) = disk
                .iter()
                .enumerate()
                .rev()
                .find(|(_, s)| s.file == Some(fileid))
                .unwrap();

            if let Some((j, fp)) = disk
                .iter()
                .enumerate()
                .find(|(_, s)| s.file.is_none() && s.len >= moved_span.len)
            {
                if j > i {
                    continue 'file;
                }
                free_span = *fp;
                free_place = j;
            } else {
                continue 'file;
            }
            moved_file_place = i;
            new_free_len = free_span.len - moved_span.len;
            free_span.len = moved_span.len;
        }
        free_span.file = Some(fileid);

        // this is not really necessary since end empty spaces won't be used to defragmentation
        if moved_file_place < disk.len() - 1 && disk[moved_file_place + 1].file.is_none() {
            disk[moved_file_place + 1].len += disk[moved_file_place].len;
            disk.remove(moved_file_place);
        } else {
            disk[moved_file_place].file = None;
        }

        disk[free_place] = free_span;
        if new_free_len > 0 {
            let new_free = Span {
                file: None,
                len: new_free_len,
            };

            disk.insert(free_place + 1, new_free);
        }
    }

    Some(
        disk.iter()
            .fold((0, 0), |(i, mut acc), span| {
                if let Some(fileid) = span.file {
                    for j in 0..span.len {
                        acc += (i + j) * fileid;
                    }
                }
                (i + span.len, acc)
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
