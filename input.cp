/*
This is a programming language designed for competitive programming.
Here's how it looks rn (could change a lot).
*/

// // comment
/* '/*' comment */
works with */ inside
*/

// function declaration: fn name(args) type {body}
fn f(int x) int {
	print(x);
	return 42;
}
// template function
fn!(T1) f(T1 a, T1 b) T1 {
	a+a+b
}

// struct declaration
struct point {
	ll x,
	ll, // with no name, you can only access by position
}
struct!(T1,T2) pair {
	T1 first,
	T2 second,
}

// variable declaration
int x;
point p;
pair!(int,int) a;

// access struct members by name or position
p.x=42;
a.0=42;
s = "42\n";

// if statements
if x*x>2 {
	print(42);
} else {
	print(42);
}
if x<0; x=0;
else {
	x++;
}
if {x++;x>2} x--;
x = (if x<0 {0} else {x});

// while
while x--; print(x);
while x-- {
	print(x);
	break; // jump to outside the loop
	continue; // jump to condition checking
	break 42; // break from 42 nested loops
	continue 42; // break 41 loop and continue
}
while {x--} {print(x);}
