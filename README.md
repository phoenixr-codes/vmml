Very Minimal Markup Language (VMML) is a lightweight markup language that supports
nested `fields` (also known as "elements" in other markup languages such as XML)
that contain an attribute.

The purpose of VMML is giving the developer the ability to define how the attribute
syntax is. Other things like escaping and capturing fields is done by the parser for
the developer.

An example of a VMML document looks as follows

```text
The [quick](bold) brown [fox](orange) jumps [over the [lazy](bold) dog](italic).
```
