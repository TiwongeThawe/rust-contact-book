Rust Contact Book Bootcamp

Level 1 - Ownership & Borrowing

> Goal: Understand ownership before collections.




---

Objectives

By the end of Level 1 you should be able to

Create a struct

Instantiate a struct

Understand ownership

Understand borrowing

Read compiler errors

Know why String is used instead of &str



---

Deliverables

[x] Create a Contact

[x] Print it

[x] Pass it to a function

[x] Trigger an ownership error

[x] Fix it using borrowing

[x] Modify a contact using a mutable borrow



---

New Concepts

Struct

A struct groups related values into one custom type.

Example

struct Contact {
    name: String,
    phone: String,
    email: String,
}

Think of it like a real contact card.

Contact

+--------------------+
| Name               |
| Phone              |
| Email              |
+--------------------+


---

String

Rust has two common string types.

&str

Borrowed string.

Usually fixed.

Does not own memory.


---

String

Owned string.

Can grow.

Lives on the heap.

Owns its memory.

Because a Contact owns its own information we use

String


---

Ownership

Every value has exactly one owner.

person

owns

name
phone
email

Only one variable owns the Contact.


---

Move

This function

show(person)

moves ownership.

Ownership becomes

main

↓

show()

↓

destroyed

When show() finishes, the Contact is dropped.

Trying to use

person

again produces an error.


---

Borrow

Borrowing means

> "You may use my data but you don't own it."



Instead of

show(person)

use

show(&person)

Ownership never leaves main.


---

Mutable Borrow

&mut Contact

allows changing data.

Example

Phone

0977....

↓

0966....

Only ONE mutable borrow can exist.


---

Debug

#[derive(Debug)]

Automatically teaches Rust how to print a struct.

Without it

println!("{:?}", contact);

will not compile.


---

Compiler Error

Error

value borrowed after move


---

Why?

Ownership moved into

show(person)

so

person

no longer exists.


---

Rust Hint

Rust even suggested

borrow instead

That is exactly what experienced Rust developers would do.


---

Fix

Change

show(person)

to

show(&person)

and

fn show(contact: Contact)

to

fn show(contact: &Contact)


---

Questions You Should Be Able To Answer

1

Who owns the Contact?

Answer:

person


---

2

Why can't I use it after calling show()?

Because ownership moved.


---

3

What does & mean?

Borrow.


---

4

Does borrowing create a copy?

No.


---

5

Why use String?

Because the Contact owns its own data.


---

Common Beginner Mistakes

❌ Thinking

&

copies data.

It doesn't.


---

❌ Using

.clone()

to silence errors.

Usually the design should borrow instead.


---

❌ Assuming functions always copy values.

Rust moves values unless borrowed.


---

What You Learned

✅ Structs

✅ Ownership

✅ Borrowing

✅ Mutable borrowing

✅ Debug

✅ Reading compiler errors


---


---

Level 2 - Collections & Searching

> Goal: Understand ownership inside a Vec.




---

Objectives

By the end of Level 2 you should understand

Vec

push()

iter()

into_iter()

Option

Borrowed return values



---

Deliverables

[x] Create a Vec<Contact>

[x] Add multiple contacts

[x] Print them

[x] Trigger a move-out-of-vector error

[x] Fix it using borrowing

[ ] Write display()

[ ] Write find_contact()

[ ] Return Option<&Contact>


(The last three are still for you to complete.)


---

New Concepts

Vec

A growable array.

Vec<Contact>

+---------+
| Alice   |
+---------+
| Bob     |
+---------+
| Charlie |
+---------+

The Vec owns every Contact.


---

push()

contacts.push(contact)

Ownership changes

contact

↓

Vec<Contact>

After pushing,

the vector owns it.


---

Indexing

contacts[0]

means

> First Contact.




---

Mistake

You wrote

let first = contacts[0];


---

Compiler Error

Rust said

cannot move out of index of Vec

and suggested

consider borrowing here

&contacts[0]

Exactly as shown in your screenshot.


---

Why?

Imagine removing Alice.

Before

Alice
Bob
Charlie

Now Alice disappears.

Who owns her?

The vector still thinks it does.

Rust refuses.


---

Fix

Instead of moving

contacts[0]

borrow

&contacts[0]

Now ownership stays inside the vector.


---

iter()

contacts.iter()

returns

&Contact

Every contact is borrowed.

Nothing moves.


---

into_iter()

contacts.into_iter()

consumes the vector.

Ownership becomes

Vec

↓

Loop

Afterwards

contacts

doesn't exist anymore.


---

Option

Instead of returning

null

Rust returns

Some(contact)

or

None

Every possibility must be handled.


---

Borrowed Return Values

Instead of returning

Contact

return

&Contact

The vector keeps ownership.

The caller only borrows.


---

Questions You Should Answer

Why does push() take ownership?

Because the vector stores the Contact.

It must own it.


---

Why can't I move from

contacts[0]

Because the vector still owns that Contact.


---

Difference between

iter()

and

into_iter()

iter()

Borrow

into_iter()

Consume


---

Why return

Option<&Contact>

instead of

Contact

Because we don't want to move the Contact out of the vector.


---

Common Beginner Mistakes

❌ Trying

contacts[0]

instead of

&contacts[0]


---

❌ Confusing

iter()

with

into_iter()


---

❌ Returning owned values when references are enough.


---

Progress Tracker

Rust Contact Book

Level 1 ✅ Ownership

Level 2 ✅ Collections (In Progress)

Level 3 ⬜ Mutable Borrowing

Level 4 ⬜ Ownership Through Functions

Level 5 ⬜ Methods & impl

Level 6 ⬜ Shared Ownership (Rc<RefCell>)


---

From now on, I'll keep this as a consistent format for every level:

📖 Theory and concepts

🎯 Objectives

✅ Deliverables

⚠️ Intentional mistakes to make

❌ Real compiler errors and what they mean

🔧 Fixes with explanations

🧠 Mental models and diagrams

💡 Common beginner mistakes

❓Review questions

📝 End-of-level summary


By the time we reach Level 6, you'll have a complete set of Markdown notes that read like a small Rust ownership handbook, built from your own experience rather than copied from a textbook.