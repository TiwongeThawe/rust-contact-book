Rust Contact Book Bootcamp

Level 3 - Mutable Borrowing & The Borrow Checker

> Status: ✅ COMPLETE



This level is where Rust stops being "a language with ownership" and starts becoming the Borrow Checker.

You experienced two of the most common compiler errors every Rust developer encounters.


---

Objectives

By the end of Level 3 you should understand:

Mutable borrowing (&mut)

Immutable borrowing (&)

iter_mut()

Borrow conflicts

Why scopes matter

Reading Rust compiler diagnostics

Updating values inside a Vec



---

Deliverables

[x] Create update_phone()

[x] Modify a contact inside a Vec

[x] Search before updating

[x] Trigger a borrow checker error

[x] Fix the borrow checker error

[x] Print values before and after update

[x] Understand immutable vs mutable borrowing



---

New Concepts


---

Mutable Borrow

&mut T

Means

> "I want permission to change this value."



Example

Contact

Phone

0960748478

↓

0966554433

Ownership never changes.

Only the value changes.


---

Immutable Borrow

&T

Means

> "I only want to read."



Nothing can change while someone is reading.


---

iter_mut()

Previously you used

iter()

which gives

&Contact

read-only.

Now you used

iter_mut()

which gives

&mut Contact

read and write.


---

The Famous Borrow Rule

Rust follows one rule.

Many Readers

OR

One Writer

Never both.

Think of a library book.

Five students

can

READ

the same book.

✔

But

One person

tries to

rewrite

the book

while

others read it.

❌

Rust refuses.


---

Mistake #1

Compiler Error

cannot find value `peron` in this scope


---

Why?

You accidentally typed

peron

instead of

person

Rust searched your scope.

It found nothing named

peron

Then it even suggested

Did you mean

person

One of Rust's nicest features is that the compiler often guesses what you intended.


---

Lesson

Compiler errors aren't your enemy.

Read every line.

Rust often tells you exactly how to fix the issue.


---

Mistake #2

Compiler Error

cannot borrow `contacts`
as mutable

because

it is also borrowed as immutable

This is the error every Rust beginner eventually sees.

Congratulations.

You've officially met the Borrow Checker.


---

Your Code

let bob_ref = find_contact(&contacts, "Bob");

update_phone(&mut contacts, "Bob", "...");

println!("{:?}", bob_ref);


---

Memory Diagram

Before update

contacts

│

├── Alice

├── Bob  <──── bob_ref

└── Charlie

Rust sees

bob_ref

↓

Bob

is still reading Bob.

Then you ask

update_phone()

to modify Bob.

Rust says

NO

Someone is still reading.


---

Why Rust Rejects This

Imagine two threads.

Thread A

Reading Bob's phone number.

-------------------

Thread B

Changing Bob's phone number.

That creates a data race.

Rust prevents it before your program ever runs.


---

Fix #1

Use the immutable borrow first.

let bob_ref = find_contact(&contacts, "Bob");

println!("{:?}", bob_ref);

update_phone(&mut contacts, "Bob", "...");

Now

Read

↓

Finished

↓

Write

No overlap.

Rust is happy.


---

Fix #2

Use a Scope

{
    let bob_ref = find_contact(...);

    println!("{:?}", bob_ref);

}

update_phone(...);

The borrow ends at the closing brace.

A surprisingly common Rust pattern.


---

Your Output

Before update:

Bob

0960748478

Then

After update

Bob

0966554433

Perfect.

That proves

search works

update works

mutable borrow succeeded



---

Warning

field

email

is never read

Ignore it.

You're not using email yet.

Once we implement

search by email

update email

delete contact


the warning disappears.


---

Mental Model

Think of borrowing like keys.

Owner

↓

Contact

Readers receive

Read Key 🔑

Many readers allowed.

Writers receive

Master Key 🔐

Only ONE master key exists.

No readers allowed while someone has it.


---

Compiler Errors Encountered

Error E0425

cannot find value

Meaning

Variable doesn't exist.

Usually

typo

wrong scope

wrong spelling



---

Error E0502

cannot borrow as mutable

because

already borrowed as immutable

Meaning

Someone is reading.

You tried writing.


---

Common Beginner Mistakes

Mistake

Using

iter()

instead of

iter_mut()

Cannot modify values.


---

Mistake

Holding a borrow longer than necessary.

Solution

Smaller scopes.


---

Mistake

Using

clone()

to silence borrow errors.

Usually incorrect.

Borrow instead.


---

Mistake

Ignoring compiler suggestions.

Rust's suggestions are often the fastest path to the fix.


---

Knowledge Check

You should now be able to answer:

Why does iter_mut() exist?

Because iter() only allows reading, while iter_mut() provides mutable references so each element can be changed in place.


---

Why can't Rust allow both borrows?

Because reading and writing the same data at the same time can lead to inconsistent or unsafe behavior. Rust prevents this at compile time.


---

Why did printing bob_ref before updating fix the problem?

Because the immutable borrow ended before the mutable borrow started, so the two no longer overlapped.


---

Why did braces fix the issue?

They limited the lifetime of the immutable borrow to that smaller scope, allowing the mutable borrow afterward.


---

What does &mut Vec<Contact> mean?

The function temporarily borrows the vector with permission to modify it, while ownership stays with the caller.


---

Level 3 Summary

You can now:

✅ Read data safely

✅ Modify data safely

✅ Understand immutable borrowing

✅ Understand mutable borrowing

✅ Understand why Rust rejects overlapping borrows

✅ Read and interpret compiler errors

✅ Update elements inside a Vec


---

Progress Tracker

Rust Contact Book Bootcamp

Level 1 ✅ COMPLETE
Ownership
Borrowing
Moving
Mutable Borrowing

Level 2 ✅ COMPLETE
Vec
push()
iter()
into_iter()
Option<&Contact>
Searching

Level 3 ✅ COMPLETE
iter_mut()
Mutable Updates
Borrow Checker
Borrow Lifetimes
Scopes
Compiler Errors (E0425, E0502)

Level 4 ⬜ Next
Ownership Through Functions

Level 5 ⬜ ContactBook Methods

Level 6 ⬜ Shared Ownership (Rc<RefCell>)

Mentor's Notes

You've reached an important milestone. By intentionally causing and then fixing E0502, you've encountered one of Rust's defining concepts. Most beginners struggle with this error because they try to fight the borrow checker. You did the opposite: you read the message, understood why it happened, and restructured your code. That's exactly the mindset that will make the later levels—especially methods, lifetimes, and shared ownership—much easier.