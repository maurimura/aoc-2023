# Day 1

Read each line and go from left to right looking for numbers. When it finds the first number, it fills a tuple ```rs(u32, u32)``` with that value on each side. After that, new numbers will change the rigt side of the tuple.

Once it reads all, it creates the two digit number from the tuple and adds that to the total.