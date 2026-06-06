
# DEFINITION:
a trait is a set of method signature, it is a contract that types promise to fulfil: it is like an interface that says: anything that implement me should be able to do this things.
they defines shared behaviour between types.

## TYPE OF POLYMORPHISM
1. Static Dispatch  compile-time polymorphism
2. Dynamic Dispatch Run-time polymorphism

# Static Dispatch
Static dispatch happens when the compiler knows the exact type at compile time

it achieves this through:
1. Generics and 
2. Traits bound

## syntax:
fn static_dispatch<T:Trait>(item:&T){
    item.behaviour();
}

## Dynamic dispatch:
Dynamic occurs when concrete type is not know an compile time





