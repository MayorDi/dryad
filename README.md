![alt][logo]

<b>_Dryad_</b> is a simple simulation of the behavior of virtual cells.

Features
---
The world, namely the grid, is represented by 3 types of segments: air, soil and cells.<br>
Air is only a space that has only physical properties that affect the habitat of cells.<br>
The soil also has physical properties, but the soil also has chemical properties, which makes the soil a rich source of resources.<br>
Cells are a key part of the simulation, they can divide, exchange energy, as well as nutrients.<br>
<br>
All cells are divided into several types:
* Photosynthetics are cells that are able to produce energy from light, while spending resources extracted from the soil;
* Conductors - cells that provide maximum metabolism and energy, consume little energy, are able to move energy quickly and with greater capacity;
* Builders are cells that ensure the growth of the body;
* Producer are cells that ensure the birth of a new individual;
* Consumer - cells acting as roots are the only cells that are able to interact with the soil directly.

``` mermaid
flowchart TD
  id1[[Air]] ==>|light|id2
  id6 -->|waste products|id7
  
  subgraph Organism
    subgraph Basic
      id2(Photosynthetics)
      id3(Conductors)
      id6(Consumer)
    end
    
    subgraph LessCritical
      id4(Builders)
      id5(Producer)
    end
  end
  
  id2 --> id2
  id2 ==> id3
  id2 -.-> id4
  id2 -.-> id5
  id2 --> id6
  
  id3 ==> id3
  id3 ==> id2
  id3 --> id4
  id3 --> id5
  id3 ==> id6
  
  id6 --> id6
  id6 ==> id3
  id6 --> id2
  
  id7[[Dirt]] ==>|water, nitrogen and so on|id6
```

[logo]: ./assets/imgs/logo.png