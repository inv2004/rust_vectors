l:1000000?1000
rep:1000

r:system"t:rep deltas l"
-1 "deltas: ",string r%rep;

r:system"t:rep dev l"
-1 "dev: ",string r%rep;

r:system"t:rep @[l;where l mod 2;*;100]"
-1 "odd_mul_amend: ",string r%rep;

r:system"t:rep 100*l where l mod 2"
-1 "odd_mul: ",string r%rep;

r:system"t:rep 100*l where 1=l mod 2"
-1 "odd_mul_cmp: ",string r%rep;

r:system"t:rep (999_ 1000 msum l)%1000"
-1 "mavg (msum): ",string r%rep;

r:system"t:rep 999_ 1000 mavg l"
-1 "mavg (999_): ",string r%rep;

r:system"t:rep 1000 mavg l"
-1 "q_mavg: ",string r%rep;

l:10000000?1000
r:system"t:rep max l"
-1 "max: ",string r%rep;

l:1000001?1000
r:system"t:rep med l"
-1 "med: ",string r%rep;

l:10000000?1000
l2:10000000?1000
r:system"t:rep l2 wavg l"
-1 "wavg: ",string r%rep;

\\
