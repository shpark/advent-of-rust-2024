use std::fmt;

use itertools::Itertools;

const INPUT: &str = include_str!("./day9.txt");

#[derive(Clone, Copy, Debug)]
enum BlockKind {
    Id(u32),
    Free,
}

#[derive(Clone, Debug)]
struct Block {
    kind: BlockKind,
    size: u32,
}

#[derive(Clone, Debug)]
struct FileSystem(Vec<Block>);

impl FileSystem {
    fn compress_once1(&mut self) -> Result<(), &'static str> {
        let tail = loop {
            if  let Some(block) = self.0.pop() {
                match block.kind {
                    BlockKind::Free => continue,
                    BlockKind::Id(_) => { break block; }
                }
            }
        };

        for i in 0..self.0.len() {
            if matches!(self.0[i].kind, BlockKind::Free) {
                let block = self.0.remove(i);

                if tail.size <= block.size {
                    self.0.insert(i, Block {
                        kind: tail.kind,
                        size: tail.size,
                    });

                    if tail.size < block.size {
                        self.0.insert(i + 1, Block {
                            kind: BlockKind::Free,
                            size: block.size - tail.size,
                        });
                    }
                } else {
                    self.0.insert(i, Block {
                        kind: tail.kind,
                        size: block.size,
                    });

                    self.0.push(Block {
                        kind: tail.kind,
                        size: tail.size - block.size,
                    });
                }

                return Ok(())
            }
        }

        self.0.push(tail);

        Err("meh")
    }

    fn compress1(&mut self) {
        while let Ok(_) = self.compress_once1() {
        }
    }

    fn compress_once2(&mut self) -> Result<(), &'static str> {
        for i in (0..self.0.len()).rev() {
            let block = self.0[i].clone();

            if matches!(&block.kind, BlockKind::Free) {
                continue;
            }

            let required = block.size;

            for j in 0..i {
                let curr = self.0[j].clone();

                match curr.kind {
                    BlockKind::Id(_) => continue,
                    BlockKind::Free => {
                        if curr.size >= required {
                            // XXX: We are not merging newly freed block and
                            // its free space neighbors...
                            let block = std::mem::replace(
                                &mut self.0[i],
                                Block {
                                    kind: BlockKind::Free,
                                    size: required,
                                });

                            let _ = self.0.remove(j);

                            self.0.insert(j, block.clone());

                            if curr.size > required {
                                self.0.insert(j + 1, Block {
                                    kind: BlockKind::Free,
                                    size: curr.size - required,
                                });
                            }

                            return Ok(());
                        }
                    }
                }
            }
        }

        Err("meheh")
    }

    fn compress2(&mut self) {
        while let Ok(_) = self.compress_once2() {
        }
    }

    fn csum(&self) -> usize {
        self.0.iter().fold((0usize, 0usize), |(pos, csum), block| {
            let next_pos = pos + block.size as usize;

            if let BlockKind::Id(id) = block.kind {
                (next_pos, csum + (id as usize) * ((pos..next_pos).sum::<usize>()))
            } else {
                (next_pos, csum)
            }
        }).1
    }
}

impl TryFrom<&str> for FileSystem {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(FileSystem(
            value.chars()
                .map(|c| c.to_digit(10).unwrap())
                .chunks(2)
                .into_iter()
                .enumerate()
                .flat_map(|(id, mut chunk)| {
                    let mut blocks = Vec::new();

                    if let Some(allocated_space) = chunk.next() {
                        blocks.push(Block {
                            kind: BlockKind::Id(id as u32),
                            size: allocated_space,
                        })
                    }

                    if let Some(free_space) = chunk.next() {
                        blocks.push(Block {
                            kind: BlockKind::Free,
                            size: free_space,
                        })
                    }

                    blocks
            })
            .collect::<Vec::<_>>()
        ))
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().for_each(|block| {
            // FIXME
            write!(f, "{}", match block.kind {
                BlockKind::Free => String::from(".").repeat(block.size as usize),
                BlockKind::Id(id) => id.to_string().repeat(block.size as usize),
            });
        });

        Ok(())
    }
}

fn main() {
    let filesystem: FileSystem = INPUT.try_into().unwrap();

    let mut fs1 = filesystem.clone();

    fs1.compress1();

    println!("{}", fs1.csum());

    let mut fs2 = filesystem.clone();

    fs2.compress2();

    println!("{}", fs2.csum());
}
