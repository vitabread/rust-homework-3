#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    class_id: u32,
}

#[derive(Debug)]
struct Club {
    id: u32,
    name: String,
    members: Vec<u32>,
}

#[derive(Debug)]
struct Class {
    id: u32,
    name: String,
    students: Vec<u32>,
}

#[derive(Debug)]
struct Course {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct StudentManagementSystem {
    students: Vec<Student>,
    clubs: Vec<Club>,
    classes: Vec<Class>,
    courses: Vec<Course>,
}

impl StudentManagementSystem {
    // 创建学生
    fn create_student(&mut self, id: u32, name: String, age: u8, class_id: u32) {
        let student = Student {
            id,
            name,
            age,
            class_id,
        };
        self.students.push(student);
    }

    // 创建社团
    fn create_club(&mut self, id: u32, name: String) {
        let club = Club {
            id,
            name,
            members: Vec::new(),
        };
        self.clubs.push(club);
    }

    // 创建班级
    fn create_class(&mut self, id: u32, name: String) {
        let class = Class {
            id,
            name,
            students: Vec::new(),
        };
        self.classes.push(class);
    }

    // 创建课程
    fn create_course(&mut self, id: u32, name: String) {
        let course = Course {
            id,
            name,
        };
        self.courses.push(course);
    }

    // 更新学生信息
    fn update_student(&mut self, id: u32, name: String, age: u8, class_id: u32) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = name;
            student.age = age;
            student.class_id = class_id;
        }
    }

    // 删除学生
    fn delete_student(&mut self, id: u32) {
        self.students.retain(|s| s.id != id);
    }

    // 添加学生到社团
    fn add_student_to_club(&mut self, student_id: u32, club_id: u32) {
        if let Some(club) = self.clubs.iter_mut().find(|c| c.id == club_id) {
            club.members.push(student_id);
        }
    }

    // 从社团移除学生
    fn remove_student_from_club(&mut self, student_id: u32, club_id: u32) {
        if let Some(club) = self.clubs.iter_mut().find(|c| c.id == club_id) {
            club.members.retain(|&id| id != student_id);
        }
    }

    // 添加学生到班级
    fn add_student_to_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(class) = self.classes.iter_mut().find(|c| c.id == class_id) {
            class.students.push(student_id);
        }
    }

    // 从班级移除学生
    fn remove_student_from_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(class) = self.classes.iter_mut().find(|c| c.id == class_id) {
            class.students.retain(|&id| id != student_id);
        }
    }

    // 获取学生信息
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }

    // 获取社团信息
    fn get_club(&self, id: u32) -> Option<&Club> {
        self.clubs.iter().find(|c| c.id == id)
    }

    // 获取班级信息
    fn get_class(&self, id: u32) -> Option<&Class> {
        self.classes.iter().find(|c| c.id == id)
    }

    // 获取课程信息
    fn get_course(&self, id: u32) -> Option<&Course> {
        self.courses.iter().find(|c| c.id == id)
    }
}

fn main() {
    let mut system = StudentManagementSystem {
        students: Vec::new(),
        clubs: Vec::new(),
        classes: Vec::new(),
        courses: Vec::new(),
    };

    // 示例操作
    system.create_student(1, String::from("Alice"), 20, 1);
    system.create_student(2, String::from("Bob"), 21, 1);
    system.create_club(1, String::from("Chess Club"));
    system.create_class(1, String::from("Math Class"));
    system.create_course(1, String::from("Calculus"));

    system.add_student_to_club(1, 1);
    system.add_student_to_club(2, 1);
    system.add_student_to_class(1, 1);
    system.add_student_to_class(2, 1);

    if let Some(student) = system.get_student(1) {
        println!("Student: {:?}", student);
    }

    if let Some(club) = system.get_club(1) {
        println!("Club: {:?}", club);
    }

    if let Some(class) = system.get_class(1) {
        println!("Class: {:?}", class);
    }

    if let Some(course) = system.get_course(1) {
        println!("Course: {:?}", course);
    }
}
