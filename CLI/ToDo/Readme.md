# **Encrypted ToDo List: The Most Secure Way to Forget Your Tasks**

Welcome to the **Encrypted ToDo List**—your to-do list, now with a *fortified security vault*, powered by the mighty and indestructible language known as **Rust**. Forget about leaving your to-do list lying around for anyone to see. This isn't just any ordinary task manager—this is a *secret agent* of productivity.

**Mission:** Keep track of your tasks, while ensuring nobody (including your future self) can snoop on them.

## **Features (The Secret Stuff)**

- **Date-based Task Tracking**: Organize your tasks by date—because let's face it, you’re probably doing everything last minute anyway. 🗓️
- **Completion Status**: Finally mark tasks as done. Don’t worry, you can lie to yourself and mark them as complete even if you haven’t done them... yet. ✅
- **Encrypted Storage**: Your tasks are encrypted so well, not even your best friend (or your dog) can break in. Not even a Rust hacker can unlock it without the password! 🔒
- **Rust-powered Efficiency**: It’s lightning fast, like a caffeinated squirrel on a mission. 🚀
- **TOML File Storage**: A human-readable format for keeping things neat—because even encrypted to-do lists deserve a little tidiness. 🧹

## **Why Should You Use This?**
- You want your tasks to be as **secret** as your midnight snack runs.
- You’ve been living a life of *unorganized chaos*, and you finally want to get your act together, but securely.
- You like **Rust** and want to show off how cool and efficient it is while managing your to-dos (or procrastinating—no judgment).
- You’re that person who likes their projects with a side of **“how do I make this complicated, but in a fun way?”**—mission accomplished.

## **How To Use It (Rust Style)**

1. **Install Rust (Because How Else Would You Run This Beast?)**
    - If you don’t have Rust installed yet, what are you waiting for? Install it from [rust-lang.org](https://www.rust-lang.org/learn/get-started).
    - Once you have Rust installed, run the following to clone and build the project:
      ```bash
      git clone https://github.com/yourusername/encrypted-todo-list.git
      cd encrypted-todo-list
      cargo build --release
      ```

2. **Add Tasks (Start Organizing, Sorta)**
    - Now that your Rust-powered to-do list is ready to go, here’s how you add tasks:
      ```bash
      ./target/release/todolist add "Buy milk" --due "2025-03-28"
      ./target/release/todolist add "Finish writing Rust code" --due "2025-03-29"
      ```

3. **Mark Tasks as Done (Or Pretend You’ve Done Them)**
    - After your most important tasks are added, mark them as done. Or, you know, *maybe* leave them incomplete for a bit longer:
      ```bash
      ./target/release/todolist complete "Buy milk"
      ```

4. **View Your Secure To-Do List (Spy-Level Access Only)**
    - Want to see your tasks? Just pull up the list, encrypted for your peace of mind:
      ```bash
      ./target/release/todolist list
      ```

5. **Decrypt the File (Because You're Not a Robot)**
    - Your encrypted file is locked tighter than your mom’s cookie jar. You'll need a password to decrypt it:
      ```bash
      ./target/release/todolist decrypt --password "supersecretpassword"
      ```

## **Security (The Only Way to Keep Your Secrets Safe)**

This project uses **AES encryption** to protect your data—just like how you keep your phone locked and guarded from the prying eyes of your friends. 

**Important Note**: Do not lose your password. Forgetting your password is like losing the key to a vault—your tasks will be forever encrypted (and probably taunting you).

- Strong passwords are a must—don't just use "password123". You’re better than that. 🦸‍♀️
- If your encrypted file gets lost, the only thing you’ll be unlocking is regret. 🥲

## **Possible Enhancements (Because It’s Never Really Finished)**

- **Notifications**: What if you could get a reminder, like “Hey, why haven’t you done anything yet?” (Okay, maybe not that aggressive.)
- **Priority Levels**: Because let’s face it, some tasks *definitely* need to be prioritized over others, like "Take a nap" vs. "Clean the garage."
- **Cloud Sync**: You could sync your encrypted to-do list across devices! Or just continue using one device (because we like simplicity).
- **Backup**: Let’s face it—losing data sucks. We’ll add a backup feature soon… or maybe tomorrow… when we feel like it. 😅

## **FAQ (Frequently Asked Questions)**

**Q: I lost my password—what now?**
- **A**: Well, that’s what you get for picking “1234” as your password. 😜 Unfortunately, if you lose it, your tasks are stuck in encrypted limbo. But hey, it’s a good lesson for next time!

**Q: Can I add emojis to my tasks?**
- **A**: Absolutely. Want your tasks to be super expressive? "🚀 Launch my career!" or "😴 Take a nap!" You do you. 💅

**Q: Why is this in Rust?**
- **A**: Because Rust is a beast. It’s fast, efficient, and doesn’t let you mess up too much. Plus, Rust gives you that "I’m a wizard" feeling every time you compile code.

**Q: Is this open-source?**
- **A**: Of course! Just don’t break the encryption... please. 🥲

---

## **Contribute (Because We Love Rust Fans)**

Have a cool feature in mind? Found a bug? Think you can make encryption even more secure? We welcome contributions, pull requests, and even jokes (as long as they’re Rust-related). Feel free to submit an issue or open a PR!

---

Thank you for checking out **Encrypted ToDo List**—your new favorite way to organize tasks while simultaneously becoming a *Rust-powered* encryption wizard. 🦸‍♂️🦾

