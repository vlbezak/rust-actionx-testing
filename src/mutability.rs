trait Task {
    fn do_it(&mut self);
}

struct Worker<'a> {
    tasks: Vec<&'a mut dyn Task>,
}

impl<'a> Worker<'a> {
    pub fn work(&mut self) {
        for task in self.tasks.iter_mut() {
            self.work_one(*task);
        }
    }

    fn work_one(&mut self, task: &mut dyn Task) {
        task.do_it();
    }
}


struct Test<'a> {
    a: &'a str,
}

impl<'a> Test<'a> {
    pub fn new() -> Self {
        Test { a: &mut "test" }
    }

    pub fn dostuff(&mut self) {
        self.a = "test";
    }

    pub fn fixme(&mut self) {
        let mut i = 0;
        while i < 10 {
            self.dostuff();
            i += 1;
        }
    }
}

fn main() {
    let mut test = Test::new();
    test.fixme();
}