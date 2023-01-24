// [asy]
size(150);
defaultpen(linewidth(0.7)+fontsize(10));

pair A,B,C,D,E,F,G,H;

A=(0,2); B=(2,2); C=(2,0); D=(0,0);
E=(1,3); F=(3,1); G=(1,1); H=(3,3);

draw(A--B--C--D--cycle);
draw(E--F--G--H--cycle);
draw(A--E--H--D);
draw(B--F--G--C);

label("1",A,W); label("2",B,N); label("3",C,E); label("4",D,S);
label("5",E,W); label("6",F,N); label("7",G,S); label("8",H,E);

fill(A--B--C--D--cycle,gray);
fill(E--F--G--H--cycle,cyan);
// [/asy]