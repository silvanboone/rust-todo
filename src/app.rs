use crate::Task;
pub struct App {
    pub task_list: Vec<Task>,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { task_list: tasks }
    }

    pub fn create_task(&mut self, desc: String, time: Option<i64>, keywords: Option<Vec<String>>) {
        let task = Task::new(desc, time, keywords);
        self.task_list.push(task);
    }

    pub fn edit_task(
        &mut self,
        desc: String,
        time: Option<i64>,
        keywords: Option<Vec<String>>,
        new_desc: String,
        new_time: Option<i64>,
        new_keywords: Option<Vec<String>>,
    ) {
        let task: Task = Task::new(desc, time, keywords);
        let new_task: Task = Task::new(new_desc, new_time, new_keywords);

        let index = self.task_list.iter().position(|t| *t == task);

        if let Some(index) = index {
            self.task_list[index] = new_task;
        }
    }

    pub fn delete_task(&mut self, desc: String, time: Option<i64>, keywords: Option<Vec<String>>) {
        let task = Task::new(desc, time, keywords);

        let index = self.task_list.iter().position(|t| *t == task);

        if let Some(i) = index {
            self.task_list.remove(i);
        }
    }

    pub fn filter_tasks(
        &self,
        desc_filter: Option<&str>,
        time_filter: Option<i64>,
        keywords_filter: Option<Vec<String>>,
    ) -> Vec<Task> {
        let mut filtered_list: Vec<Task> = vec![];

        for i in self.task_list.iter() {
            if let Some(desc_filter) = desc_filter {
                if i.desc.contains(desc_filter) {
                    filtered_list.push(i.to_owned());

                    continue;
                }
            }
            if let Some(time_filter) = time_filter {
                if let Some(time) = &i.time {
                    if time == &time_filter {
                        filtered_list.push(i.to_owned());
                        continue;
                    }
                }
            }

            // check if there is a filter for keywords
            if let Some(ref keywords_filter) = keywords_filter {
                // check if task has keywords
                if let Some(keywords) = &i.keywords {
                    // check if there are matching keywords
                    if keywords_filter.iter().any(|f| keywords.contains(&f)) {
                        filtered_list.push(i.to_owned());

                        continue;
                    }
                }
            }
        }

        return filtered_list;
    }

    // pub fn sort_tasks() -> Vec<Task> {}
}
