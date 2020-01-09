puzzle(X) :-
    X = [A,B,C,D,E],
    member(2, X),
    member(3, X),
    member(5, X),
    member(7, X),
    member(9, X),
    399 is A + (B * (C^2)) + (D^3) - E.
