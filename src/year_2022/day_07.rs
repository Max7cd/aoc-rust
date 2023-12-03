use std::{collections::HashMap, rc::Rc, cell::RefCell};

#[macros::aoc_day]
pub struct State {
    filesystem: File
}

#[derive(Debug, Clone)]
enum File {
    File(u64),
    Dir(HashMap<String, File>)
}

impl File {
    fn get_size(&self) -> u64{
        match self {
            File::File(size) => size.clone(),
            File::Dir(hm) => {
                hm.iter().map(|(_path, file)| file.get_size() ).sum::<u64>()
            }
        }
    }

    fn insert(&mut self, path: &Vec<String>, name: &String, file: File) {
        if let File::Dir(hm) = self {
            if path.is_empty() {
                hm.insert(name.to_string(), file);
            } else {
                hm.get_mut(path.first().unwrap()).unwrap().insert(&path[1..].to_vec(), name, file);
            }
        }
    }
}

struct NumWrapper {
    i: u64
}

struct InternalState {
    needed: u64,
    smallest_match: u64
}

fn setup(puzzle_input: Vec<String>) -> State {
    let mut fs: File = File::Dir(HashMap::new());
    let mut current_dir: Vec<String> = vec![];
    for line in puzzle_input.iter() {
        if line.starts_with("$ cd /"){
            current_dir.clear();
        } else if line.starts_with("$ cd .."){
            current_dir.pop();
        } else if line.starts_with("$ cd"){
            let split= line.split(" ");
            let name : String = split.skip(2).next().unwrap().to_owned();
            current_dir.push(name.to_string());
        } else if line.starts_with("dir") {
            let split= line.split(" ");
            let name : String = split.skip(1).next().unwrap().to_owned();
            fs.insert(
                &current_dir,
                &name,
                File::Dir(HashMap::new())
            );
        } else if line.starts_with("$ ls") {

        } else {
            let mut split= line.split(" ");
            let size : u64 = split.next().unwrap().parse().unwrap();
            let name : String = split.next().unwrap().to_owned();
            fs.insert(
                &current_dir,
                &name,
                File::File(size)
            );
        }
    }
    State { filesystem: fs }
}


fn puzzle_01(state: State) -> AOCResult {
    let total : Rc<RefCell<NumWrapper>> = Rc::new(RefCell::new(NumWrapper {i:0}));
    fn func(it: &File, total: Rc<RefCell<NumWrapper>>) {
        if let File::Dir(hm) = it {
            let mut dir_total : u64 = 0;
            for (_, v) in hm {
                match &v {
                    File::File(size) => dir_total += size,
                    File::Dir(_) => {
                        dir_total += v.get_size(); 
                        func(v, total.clone())
                    }
                }
            }
            if dir_total <= 100_000 {
                let mut x = total.borrow_mut();
                x.i += dir_total;
            }
        }
    }
    func(&state.filesystem, total.clone());
    let x = total.borrow().i;
    x.into()
}

fn puzzle_02(state: State) -> AOCResult {
    let used = state.filesystem.get_size();
    let needed = 30_000_000 - (70_000_000 - used);

    let internal_state : Rc<RefCell<InternalState>> = Rc::new(RefCell::new(InternalState { needed: needed, smallest_match : 70_000_000}));
    fn func(it: &File, internal_state: Rc<RefCell<InternalState>>) {
        if let File::Dir(hm) = it {
            let mut dir_total : u64 = 0;
            for (_, v) in hm {
                match &v {
                    File::File(size) => dir_total += size,
                    File::Dir(_) => {
                        dir_total += v.get_size(); 
                        func(v, internal_state.clone())
                    }
                }
            }
            if dir_total >= internal_state.borrow().needed && internal_state.borrow().smallest_match > dir_total {
                internal_state.borrow_mut().smallest_match = dir_total;
            }
        }
    }
    func(&state.filesystem, internal_state.clone());
    let out = internal_state.borrow().smallest_match.to_string();
    out.into()
}




