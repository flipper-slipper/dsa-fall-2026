# Homework Assignment #1: Hello World and Getting to Know You
## Tour (Rust)
Over the summer, I went through the entire Rust book, which is one big tutorial on Rust. 
* Features I like about Rust: It's really hard to write bad Rust code because the compiler checks for everything. It's statically typed which is comforting. The cargo package manager is just as good as pip. No cpp yuck of writing build scripts and using cmake. 

* I've found everything I've looked for. 

* Questions: It's not so much a question rather than general confusion when getting to complicated functions in docs.rs pages. 

* Debugger: I got CodeLLDB working on my Rust project. I've used debuggers before but not on Rust. For this debugger, I learned how to set up the config file. I was able to launch it and set a breakpoint. 


## Learning Strategies
When embarking on a course, it’s important to strategize about how to get the most out of the experience. One might focus on how to get the best grade, but I invite you to consider the task of trying to learn as much as possible as being the primary objective (and other rewards will follow from this goal).

Respond to the following prompts to get you thinking about your approach to learning in this course.

1. Choose a moment in your educational career (it could be an assignment or a full course) where learning went really well. What strategies did you employ that worked particularly well (e.g., working with others, trying work on your own before asking a friend, going to office hours)?
    * Learning went really well for my PIE project, Scribblz, where we built a robot that sucks to a window using an impeller. The software portion used concepts I learned in CompRobo. Being motivated about the challenge was super powerful here. There was a genuine interest to solve the problem and I felt it could have real world impacts. As long as I'm able to see some use come out of what I'm learning, I'll be ok. 

2. Similar to (1), which sorts of strategies have led to either less effective learning or less enjoyment of the learning experience. Feel free to describe a few examples of what doesn’t work for you.
    * I don't enjoy learning when I see no application of the material in my future life. Learning contrived leetcode problems that can be done by AI annoy me--when am I actually ever going to need to solve zigzag in real life. I understand they are drills, but when there is no broader impact of the material, I simply get demotivated and lost. AI has made that worse because it can basically do everything, however, I am motivated to learn DSA because I want to tell AI what algorithms to use--I still want to design code. 

3. As this course is foundational for many aspects of computer science, the problems in this class can be easily solved with modern AI systems (e.g., ChatGPT, Gemini, etc.). One of my foundational assumptions is the process of grappling with a problem helps you internalize the important concepts, gives you more insight into how the tools you are learning can be applied in other contexts, helps you more realistically assess your own abilities, and helps you learn to better communicate your knowledge to others. Particular methods of using AI (e.g., prompting the AI to provide answers to questions and thoughtlessly copying the answers) are unlikely to achieve the learning goals articulated previously. Do you agree with this framing? How are you thinking about AI tools with respect to this course?
    * I definitely agree with this framing. This is a dangerous concept to be honest. My goal for this course is to struggle with the problems. I'll commit to spending hours working on a short function so I can challenge my brain muscles. Copy and pasting AI code is just too demotivating for me. I plan to use AI to help me with Rust syntax issues and creating frameworks for how to write production-level code, but I will not use it to implement algorithms. I can use it to explain concepts, but I really want to do things by myself. 

4. What strategies will you use in this course to be successful? With respect to AI, what principles or strategies will you use during this course.
     * Thinking is important! I will think. One thing I found that works for me is simply setting a 15-minute timer of no AI use. After the timer, I can start asking for help, but only to debug--never to write entirely. I'm less concerned about syntax, so I think I'll mostly start writing code in english. I'm ok with AI helping me nail down syntax. I'll also watch YouTube videos. I find asking AI to explain concepts to me okay, but sometimes it never beats YT videos. 

5. What do you think of some of the proposed activities for the oral quizzes? Are these activities ones that would give you helpful feedback as to how you are performing with respect to the course material? Would you add or subtract any of the proposed activities?
    * I like the activities, especially because it motivates me more to use my actual intelligence. It's much more interesting to work through a problem out loud. Also, like I've mentioned before, syntax is the low-hanging fruit. The verbal quizzes focus more on approach and thinking, not irrelevant syntax. I like all the activities. Maybe you could add broader architecture problems. 

6. How can the teaching team support you?
    * Be patient when I get confused! It takes a long time sometimes. 

# Porting Code

The original Python file is a rock-paper-scissors game.

## Running it

```sh
cargo run 
cargo test 
```

## Translating from Python to Rust - Reflection

### The Good 
In rust, classes don't exist. You split the class into a struct and impl block. It's a cleaner abstraction when inheritance gets messy, because if you want to add new behaviors, you simply add more function impls. There is no inheritance in Rust. Traits Display are the only way to share behavior across types. 

Explicit typing is another thing I love. Every function signature says exactly what goes in and what comes out, so the compiler catches a mismatched type immediately instead of it surfacing three function calls later as a runtime error. 

### The Bad
Because rust is so safe, it makes you prove exhaustivness in branches. Python doesn't care, which makes python faster to write.

The rust borrow checker is the other annoyance. In Python, you just pass an object around and mutate it wherever. In Rust, something as simple as mutating the number generator means threading `&mut impl Rng` through function signatures and thinking about who owns what and for how long. It's the compiler forcing you to think about ownership and mutation--like a need to know basis. 

Error handling is in between good and bad. Python leans on exceptions: you write the happy path and wrap the risky part in a `try/except` if you feel like it, and if you don't, the exception just propagates up and crashes loudly wherever it wants. Rust doesn't have exceptions, and every failable thing returns Options or Results. The compiler basically forces you to handle these types. It's much cleaner than the nested try excepts, though the bad part is that it takes so much time. Think of it as a tradeoff between debugging at compile time vs debugging at runtime. 

# Meeting Scheduler 
How will the running time grow with n? 

For the pairs, a nested loop checking every pair of meetings possible means the outer loop runs n times and the inner loop runs up to n times, which means it grows with O(n^2). 

For the sorting, the sort itself is O(nlogn), and then the next pass over the meetings is at most one iteration, or O(n). So overall, the running time will grow by the dominant term, which is O(nlogn). This scales better than the pairs method. 