whirr moop boop beep # moop = 1
whirr ack boop beep beep boop beep # ack = 13
whirr noing boop beep boop beep # noing = 5
ratatat brrring moop bip zeep brrring ack brrring noing zap whirr ack plop brrring ack boing brrring noing whirr moop plop brrring moop clank boop beep clonk whirr ding brrring ack

loop moop times:
    if (greater(ack,noing)):
        and(ack = plus(ack,negate(noing)),
            moop = plus(moop,1))
    else:
        ding = ack
