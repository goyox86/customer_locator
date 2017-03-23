
# Design

First of all apologies in advance if the fact that the language choice makes
a bit difficult to evaluate the implementation. I know Rust is not the most
common language on solving exercises. Being said that if you know Rust then
it's just awesome! <3

## Why Rust?

I could say that I picked Rust because is an amazing language, but I'm biased.
That is why I'm going with the longer version. When I saw the exercise it
involved things that made me think about Rust. First of all it's a CLI tool,
the Rust story on that aspect is great, multitude of libraries and CLI
frameworks. There is serialization/deserialization of JSON which is desirable
to be fast as IO almost always represent a big fraction of the runtime of
anything that can be useful, as in, you spend most of the time doing IO. JSON
is not the friendliest format to be parsed by machines so I discarded other
high level languages as I wanted it to be fast so you could load very big JSON
files with a low memory footprint. The fact that it parses potentially 
untrusted data made me think on the security aspect, hence an attacker controlling
the input could try to exploit vulnerabilities which are simply not possible in Rust 
(I was thinking on doing it in Ruby but JSON parsing in Ruby if you wnat it fast you 
end up using a gem that is implemented as a C entension, and there is where the 
vulnerability could arise). Along with all of that you will see that despite being
a language that gives you a lot of control it has very high level constructs that 
make it feel like a higher level language, and the implementation is just a few lines
including tests and error handling. Iterators, Macros and Custom derives
(that allow toll free derived implementation of the JSON serializing/deserializing),
Type Classes and a very powerful type system with traits that are similar to 
interfaces in other languages that enable you to open the withdow for extension
even though enforcing a contract.

## Design

The design is very simple. First of all. There are some core entities related to
the data model:

- Location: Abstracts the concept of a location on earth surface given latitude,
longitude and responsible of calculate distances with other locations. Here is
where the Haversine implementation lives in.
- Customer: Holds each of the customers that are in the source. It also holds
convenience methods to calculate distance from a give location. Implements
serializing/deserializing of JSON (for free a library does it).
- Kilometers: An interesting one, a NewType that allows us to enforce that all
calculations are performed with the correct interpretation of the units.

There is another set of entities that were added in order to make the design
extensible and also maintainable:

- CustomerLocator: Is the entry point for loading customers and then make
calculations on them while keeping them buffered in memory. Things like
"filtering the customers within a given radius" go here.

- CustomerList: Is the currency of the system it is used whenever the
concept of a "list of customers" needs to be returned, stored or passed
around. It is similar to "Kilometers" in the sense that is a NewType that
hides the concrete implementation of the data structure that holds the
customers allowing it to be swapped without propagating a cascade of
changes across module boundary.

- CustomerDatasource: This is the interface that will allow the system to
support many different formats without requiring any change on the rest of it.
Just implement this interface/trait and you can build locators with it 
straight away.

And finally "CustomerJsonFile" which is a concrete implementation of
"CustomerDatasource" which knows how to read JSON files containg customer
data.

# File Layout and Structure

It was factored in such a way that can be easily turned on to a library
and be distributed as a crate in the future. This is in big part because
of Rust's `cargo` package manager and conventions around packaging and
libraries.

 # Closing thoughts

 I think that with this design would be a good start for bootstraping a
 framework to implement customer filtering based (but not limited) on
 files on diverse formats, in a fast, scalable and secure manner.