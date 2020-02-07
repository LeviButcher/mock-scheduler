---
author: Levi Butcher
title: Operating Systems
subtitle: Assignment 2
---

# Results

## Priority Scheduling

CPU_QUANTUM_TIME: 4 CONTEXT_SWITCH:0
| Name | Avg Turnaround Time | Waiting Time |
| --|--|--|--|
| FiFo | 213 | 171 |
| Shortest Next | 152 | 110 |
| Shortest Remaining | 150 | 108 |

CPU_QUANTUM_TIME: 4 CONTEXT_SWITCH:1
| Name | Avg Turnaround Time | Waiting Time |
| --|--|--|--|
| FiFo | 242 | 200 |
| Shortest Next | 174 | 132 |
| Shortest Remaining | 172 | 130 |

CPU_QUANTUM_TIME: 8 CONTEXT_SWITCH:4
| Name | Avg Turnaround Time | Waiting Time |
| --|--|--|--|
| FiFo | 171 | 129 |
| Shortest Next | 163 | 121 |
| Shortest Remaining | 139 | 97 |

## Without Priority Scheduling

CPU_QUANTUM_TIME: 4 CONTEXT_SWITCH:0
| Name | Avg Turnaround Time | Waiting Time |
| --|--|--|--|
| FiFo | 245 | 203 |
| Shortest Next | 171 | 129 |
| Shortest Remaining | 165 | 123 |

CPU_QUANTUM_TIME: 4 CONTEXT_SWITCH:1
| Name | Avg Turnaround Time | Waiting Time |
| --|--|--|--|
| FiFo | 307 | 265 |
| Shortest Next | 214 | 172 |
| Shortest Remaining | 206 | 164 |

CPU_QUANTUM_TIME: 8 CONTEXT_SWITCH:4
| Name | Avg Turnaround Time | Waiting Time |
| --|--|--|--|
| FiFo | 297 | 255 |
| Shortest Next | 233 | 191 |
| Shortest Remaining | 209 | 167 |

# Q & A

### Were there any drastic changes, or anything that didn’t change that you expected to change with each algorithm? What about when context switching was added? Why?

I think the results are pretty much what I expected.
FIFO is absolutely terrible and ShortestNext and Shortest Remaining are pretty close on both their turnaround and waiting times averages. What is surprising is how Shortest Remaining was practically unaffected by having a quantum_time of 8 and a context switch of 4, while the other algorithms grew a good bit in their averages.

### How did the runtime results differ with the priority queueing?

Priority queueing made the averages go down by a good bit. But it makes sense since process are being able to run longer without having to be wait through the queue again.
