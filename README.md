# Sedaro Sim

### What is it?

* **Nothing to be impressed by.**
* **A simple port of the sedaro-nano simulation to the Rust language.**
* **Moore's law increase in efficiency... probably... because of the compile.**
* **Not an improvement in complexity (Big O) as discussed below.**

* **More time was spent thinking about optimizing QRangeStore, than was spent on the porting of the code to Rust.**

### Setup and Running

* **Open a browser to http://localhost:3000/ and wait for screen updates.**
* **In the main source directory (sedaro-sim) type ```docker compose up```.**
* **Say ***Doh!*** - because you forgot to install and run Docker.**

### Troubleshooting

* **Just wait for the presentation and demo.**
* **There will be a presentation and demo... right?**
* **Call Tom (740) 707-2711 and say "what the heck?".**

### Notes on the sedaro-nano simulation, QRangeStore, etc.
* ***these are just thoughts for discussion***

* **Even though QRangeStore is created as a KV store (in Python) it is searched as a vector i.e., the __getitem__() function iterates through the key,values (in the Python comprehension) rather than jumping to a hashed or calculated value, as in a hashMap implementation.**

* **The problem is, you are searching for a "key" that is not in the KV store, it is in a range which is in the KV store. This is an age-old problem (fuzzy searching) which may be optimized with binning, possibly.**

* **Another problem might be that the range tuples which are used as keys are not necessarily unique and they can overlap. So they are not conducive to being keys.**
* **While not necessarily unique, the ranges are unique in the example simulation, although they do indeed overlap, because of random() in the return from propagate(). We may be able to use this fact for some optimization**

* ***Using binning to optimize fuzzy searching***
* If you use fixed-size bins for the ranges, then you know (a priori) which bin any floating point key value belongs in.  Then you can really use the map and jump right to the QRange (bin).**
* **This may be a valuable speed-up, even in the Python code, by reducing the big O complexity.**
* **It may also eliminate the timeStep parameter, which seems to be part of the "simulation" and not part of the "state".**
* **This binning can be accomplished using floor() and ceil() or by rounding.**

* **I don't know if fixed size bins work for this particular problem.**  
* **Why do you need variable sized ranges?  Do the ranges need to overlap?**

* **If you can live with fixed-size bins, then you may not need bins at all... bear with me...**

* **Another option:**
* **If the timeSteps (and starting times)  are fixed then you are only storing/searching for a finite number of *specific* times.**
* **so you wouldn't need to use any kind of range store...**
* **Just store a Key (floating point) with a value (state object)...**
* **The "con" is that you could be storing a given state object multiple times, for a waste of memory.**
* **Why do states only change with the given ranges?**
* **Also, what is the purpose for the random() in the propagate() function?**
