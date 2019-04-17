# Report
<p style="text-align: center">
Aleksander Wawrzyniak</br>
April, the 10th 2019
</p>

## Page Replacement Algorithms
In an operating system that uses paging for memory management, a page replacement algorithm is needed to decide which page needs to be replaced when new page comes in.

**Page Fault** - A page fault happens when a running program accesses a memory page that is mapped into the virtual address space, but not loaded in physical memory.

Since actual physical memory is much smaller than virtual memory, page faults happen. In case of page fault, Operating System might have to replace one of the existing pages with the newly needed page. Different page replacement algorithms suggest different ways to decide which page to replace. The target for all algorithms is to reduce the number of page faults.


### Page replacement algorithms
* #### First In First Out (FIFO)
**First In First Out** is the simplest page replacement algorithm. In the algorithm, operating system stores the pages as a queue, and when replacement is needed -  replaces the page that haven't been replaced the longest.

**FIFO** do not require any additional information about the pages used, beside the queue they are in, which from finding page to replace point of view might be the fastest.

```
<!-- todo -->
```

* #### Optimal Page replacement (OPT)
**Optimal Page replacement** is an algorithm, where the page, that will not be used for the longest in the future, will be replaced, which makes it the *optimal* algorithm.

Unfortunately, **OPT** cannot be implemented in general purpose operating systems, because it is impossible to reliably compute how long it will take before page is going to be used, except when all software that will run on a system is either known beforehand and is amenable to static analysis of its memory reference patterns, or only a class of applications allowing run-time analysis.

```
<!-- todo -->
```

* #### Least Recently Used (LRU)
**Least Recently Used** page replacement algorithm keeps track of the most used algorithms during a short period of time *with idea that if a page was heavily used in the recent past*.

In theory, **LRU** can provide near optimal performance, but it is rather expensive to implement - at each page miss, system have to figure out which page to replace which requires it to check all pages, when they were used.

```
<!-- todo -->
```

* Approximative Least Recently Used (ALRU)
Personally, I don't know what this algorithm is, therefore I will assume that is the **Not Recently Used** algorithm.

**NRU** is similar to the LRU, however it doesn't require checking when the page was referenced.

**Not Recently Used** sets the page as *referenced* for the next reference check whenever the page was modified or referenced, so it will not be replaced if replacement is needed.
```
<!-- todo -->
```

* #### Random (RAND)
**Random** replacement algorithm replaces a random page from the pages in the system when the page miss appears. Just like that.

```
<!-- todo -->
```


## Simulation results:
Each time all algorithms were supplied with the same set of randomly generated references.

<!-- todo -->
* ### pages: 3 requests: 200
#### Page misses:
|       | Run 1 | Run 2 | Run 3 |
| :---: | :---: | :---: | :---: |
| FIFO  |       |       |       |
| LRU   |       |       |       |
| ALRU  |       |       |       |
| OPT   |       |       |       |
| RAND  |       |       |       |

* ### pages: 5 requests: 200
#### Page misses:
|       | Run 1 | Run 2 | Run 3 |
| :---: | :---: | :---: | :---: |
| FIFO  |       |       |       |
| LRU   |       |       |       |
| ALRU  |       |       |       |
| OPT   |       |       |       |
| RAND  |       |       |       |

* ### pages: 8 requests: 300
#### Page misses:
|       | Run 1 | Run 2 | Run 3 |
| :---: | :---: | :---: | :---: |
| FIFO  |       |       |       |
| LRU   |       |       |       |
| ALRU  |       |       |       |
| OPT   |       |       |       |
| RAND  |       |       |       |

* ### pages: 6 requests: 350
#### Page misses:
|       | Run 1 | Run 2 | Run 3 |
| :---: | :---: | :---: | :---: |
| FIFO  |       |       |       |
| LRU   |       |       |       |
| ALRU  |       |       |       |
| OPT   |       |       |       |
| RAND  |       |       |       |


## Conclusions
<!-- todo -->

## Bibliography
https://en.wikipedia.org/wiki/Page_replacement_algorithm

https://www.geeksforgeeks.orgpage-replacement-algorithms-in-operating-systems/

http://www.liralab.it/teaching/OS/files_current/class_6_10.pdf