// tim_sort_without_gallop

use std::{f32::MIN, isize};

use super::bin_insertion_sort::bin_insertion_sort;

const MIN_MAERGE: usize = 64;

struct SortState<'a> {
    list: &'a mut [i32],
    runs: Vec<Run>,
    pos: usize,
}

#[derive(Debug, Copy, Clone)]
struct Run {
    pos: usize,
    len: usize,
}

struct MergeLo<'a> {
    list_len: usize,
    first_pos: usize,
    first_len: usize,
    second_pos: usize,
    dest_pos: usize,
    list: &'a mut [i32],
    temp: Vec<i32>,
}

struct MergeHi<'a> {
    first_pos: isize,
    second_pos: isize,
    dest_pos: isize,
    list: &'a mut [i32],
    temp: Vec<i32>,
}

// 计算最小 run 长度
fn calc_minrun(len: usize) -> usize {
    let mut r = 0;
    let mut new_len = len;
    while new_len >= MIN_MAERGE {
        r |= new_len & 1;
        new_len >>= 1;
    }

    new_len + r
}

fn count_run(list: &mut [i32]) -> usize {
    let (ord, pos) = find_run(list);
    if ord {
        list.split_at_mut(pos).0.reverse();
    }

    pos
}

fn find_run(list: &[i32]) -> (bool, usize) {
    let len = list.len();
    if len < 2 {
        return (false, len);
    }

    let mut pos = 1;

    if list[1] < list[0] {
        while pos < len - 1 && list[pos + 1] < list[pos] {
            pos += 1;
        }
        (true, pos + 1)
    } else {
        while pos < len - 1 && list[pos + 1] >= list[pos] {
            pos += 1;
        }
        (false, pos + 1)
    }
}

impl<'a> SortState<'a> {
    fn new(list: &'a mut [i32]) -> Self {
        SortState {
            list: list,
            runs: Vec::new(),
            pos: 0,
        }
    }

    fn sort(&mut self) {
        let len = self.list.len();

        let minrun = calc_minrun(len);

        while self.pos < len {
            let pos = self.pos;
            let mut run_len = count_run(&mut self.list
                                                        .split_at_mut(pos)
                                                        .1);
            let run_minlen = if minrun > len - pos {
                len - pos
            } else {
                minrun
            };

            if run_len < run_minlen {
                run_len = run_minlen;
                let left =  self.list
                .split_at_mut(pos).1
                .split_at_mut(run_len).0;
                bin_insertion_sort(left);
            }

            self.runs.push(Run {
                pos: pos,
                len: run_len,
            });

            self.pos += run_len;

            self.merge_collapse();
        }

        self.merge_force_collapse();
    }

    fn merge_collapse(&mut self) {
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n = runs.len() - 2;

            if (n >= 1 && runs[n - 1].len <= runs[n].len + runs[n + 1].len)
            || (n >= 2 && runs[n - 2].len <= runs[n].len + runs[n - 1].len) {
                let (pos1, pos2) = if runs[n - 1].len < runs[n + 1].len {
                    (n - 1, n)
                } else {
                    (n, n + 1)
                };

                let (run1, run2) = (runs[pos1], runs[pos2]);
                debug_assert_eq!(run1.pos + run1.len, run2.pos);

                runs.remove(pos2);
                runs[pos1] = Run {
                    pos: run1.pos,
                    len: run1.len + run2.len,
                };

                let new_list = self.list
                .split_at_mut(run1.pos).1
                .split_at_mut(run1.len + run2.len).0;
                merge_sort(new_list, run1.len, run2.len);
            } else {
                break;
            }
        }
    }

    fn merge_force_collapse(&mut self) {
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n =  runs.len() - 2;

            let (pos1, pos2) = if n > 0 &&
            runs[n - 1].len < runs[n + 1].len {
                (n - 1, n)
            } else {
                (n, n + 1)
            };
    
            let (run1, run2) = (runs[pos1], runs[pos2]);
            debug_assert_eq!(run1.len, run2.pos);

            runs.remove(pos2);
            runs[pos1] = Run {
                pos: run1.pos,
                len: run1.len + run2.len,
            };

            let new_list = self.list
            .split_at_mut(run1.pos).1
            .split_at_mut(run1.len + run2.len).0;
            merge_sort(new_list, run1.len, run2.len);
        }
    }
}

fn merge_sort(
    list: &mut [i32],
    first_len: usize,
    second_len: usize,
) {
    if 0 == first_len || 0 == second_len { return; }

    if first_len > second_len {
        merge_hi(list, first_len, second_len);
    } else {
        merge_lo(list, first_len);
    }
}

fn merge_lo(
    list: &mut [i32],
    first_len: usize,
) {
    unsafe {
        let mut state = MergeLo::new(list, first_len);
        state.merge();
    }
}

impl <'a> MergeLo<'a> {
    unsafe fn new(list: &'a mut [i32], first_len: usize) -> Self {
        let mut ret_val = MergeLo {
            list_len: list.len(),
            first_pos: 0,
            first_len,
            second_pos: first_len,

            dest_pos: 0,

            list,
            temp: Vec::with_capacity(first_len),
        };

        ret_val.temp.set_len(first_len);
        for i in 0..first_len {
            ret_val.temp[i] = ret_val.list[i];
        }

        ret_val
    }

    // 归并排序
    fn merge(&mut self) {
        while self.second_pos > self.dest_pos
         && self.second_pos < self.list_len {
            debug_assert!((self.second_pos - self.first_len) + self.first_pos == self.dest_pos);
            
            if self.temp[self.first_pos] > self.list[self.second_pos] {
                self.list[self.dest_pos] = self.list[self.second_pos];
                self.second_pos += 1;
            } else {
                self.list[self.dest_pos] = self.temp[self.first_pos];
                self.first_pos += 1;
            }
            self.dest_pos += 1;
         }
    }
}

impl<'a> Drop for MergeLo<'a> {
    fn drop(&mut self) {
        unsafe {
            if self.first_pos < self.first_len {
                for i in 0..(self.first_len - self.first_pos) {
                    self.list[self.dest_pos + i] = self.temp[self.first_pos + 1];
                }
            }

            self.temp.set_len(0);
        }
    }
}

fn merge_hi(
    list: &mut [i32],
    first_len: usize,
    second_len: usize,
) {
    unsafe {
        let mut state = MergeHi::new(list, first_len, second_len);
        state.merge();
    }
}

impl<'a> MergeHi<'a> {
    unsafe fn new(
        list: &'a mut [i32],
        first_len: usize,
        second_len: usize
    ) -> Self {
            let mut ret_val = MergeHi {
                first_pos: first_len as isize - 1,
                second_pos: second_len as isize - 1,
                dest_pos: list.len() as isize - 1,
                list,
                temp: Vec::with_capacity(second_len),
            };

            ret_val.temp.set_len(second_len);
            for i in 0..second_len {
                ret_val.temp[i] = ret_val.list[i + first_len];
            }

            ret_val
    }

    fn merge(&mut self) {
        while self.first_pos < self.dest_pos 
        && self.first_pos >= 0 {
            debug_assert!(self.first_pos + self.second_pos + 1 == self.dest_pos);
            
            if self.temp[self.second_pos as usize] >= self.list[self.first_pos as usize] {
                self.list[self.dest_pos as usize] = self.temp[self.second_pos as usize];
                self.second_pos -= 1;
            } else {
                self.list[self.dest_pos as usize] = self.list[self.first_pos as usize];
                self.first_pos -= 1;
            }
            self.dest_pos -= 1;
        }
    }
}

impl<'a> Drop for MergeHi<'a> {
    fn drop(&mut self) {
        unsafe {
            if self.second_pos >= 0 {
                let size = self.second_pos + 1;
                let src = 0;
                let dest = self.dest_pos - size;
                for i in 0..size {
                    self.list[(dest + i) as usize] = self.temp[(src + i) as usize];
                }
            }
            self.temp.set_len(0);
        }
    }
}

pub fn tim_sort(list: &mut [i32]) {
    if list.len() < MIN_MAERGE {
        bin_insertion_sort(list);
    } else {
        let mut sort_state = SortState::new(list);
        sort_state.sort();
    }
}

#[cfg(test)]
mod tests {
    use crate::random_vec;

    use super::*;

    #[test]
    fn test_tim_sort() {
        let mut list = random_vec(300, 300, 0);

        tim_sort(&mut list);
        println!("after sort: {:?}", list);
    }
}