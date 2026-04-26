Historians use specialized graph-based and entity-relationship models designed to handle the complexity of time and geography. These models move beyond simple spreadsheets to capture how people, places, and events were interconnected. [1, 2, 3]

## 1. Graph & Entity Models for History

The most widely used standard for connecting historical entities is CIDOC CRM (Conceptual Reference Model). Unlike standard business databases, it is "event-centric," meaning it uses events as the "glue" that binds everything else together. [4, 5]

- Event-Centric Structure: Instead of a direct link between a Person and a Place, the model records an _Event_ (e.g., a "Birth" or "Battle") that happened at a certain _Time_ and _Place_, involving specific _Actors_.
- Knowledge Graphs: Projects increasingly use Knowledge Graphs (like EventKG) to map historical figures. This allows for complex queries, such as "Which authors were born in the same decade and city as a specific explorer?".
- Ontologies: Tools like STOLE or PRiSMHA provide pre-defined "vocabularies" for historical data, ensuring that when two historians record a "legal record," they are using the same digital definitions for roles and relationships. [1, 5, 6, 7, 8, 9]

## 2. Scientific Methods for Viewing Metadata

"Scientific" or computational history (often called Digital History) uses several methods to analyze metadata for patterns that aren't visible to a human reader.

- Historical Network Analysis (HNA): This applies mathematical graph theory to historical metadata. For example, by mapping the metadata of letters (sender, recipient, date), historians can identify the most influential "nodes" in a 17th-century spy network or intellectual circle.
- Prosopography (Group Biography): Historians aggregate metadata from thousands of individuals to create a "collective biography." This allows them to see if a certain demographic (e.g., specific age or social class) was more likely to participate in an event like a revolution.
- Evolution Spectrographs: These are specialized visualizations that show how topics or associations in historical documents change over time. They look like heatmaps, showing where collaboration or specific terms "heated up" or faded away across decades.
- Geospatial Analysis (GIS): Metadata with latitude/longitude is processed through Historical GIS to see how human behavior changed according to terrain or distance—such as how a plague spread along specific trade routes. [7, 10, 11, 12, 13]

## Comparison of Modeling Approaches

|Feature [1, 3, 14]|Relational (SQL)|Graph/Knowledge Graph|
|---|---|---|
|Focus|Tables and predefined rows|Relationships and connections|
|Time Handling|Hard to track changes over time|Can represent evolving "states" easily|
|Best For|Structured archives, census data|Social networks, complex cause-and-effect|

Are you looking for a specific software tool to start building your own historical graph, or would you like to see a visual example of a CIDOC CRM map?

  

[1] [https://www.meegle.com](https://www.meegle.com/en_us/topics/knowledge-graphs/knowledge-graph-for-historical-data)

[2] [https://medium.com](https://medium.com/@brian-curry-research/entity-graphs-how-to-develop-analyze-and-visualize-relationships-in-the-age-of-ai-a46c6708a188)

[3] [https://datawalk.com](https://datawalk.com/relational-model-database-vs-the-datawalk-knowledge-graph/#:~:text=First%2C%20knowledge%20graphs%20excel%20in%20structuring%20data,it%20when%20new%20aspects%20and%20data%20appear.)

[4] [https://www.youtube.com](https://www.youtube.com/watch?v=_SdJvXyZ-Y0&t=12)

[5] [https://openhumanitiesdata.metajnl.com](https://openhumanitiesdata.metajnl.com/articles/10.5334/johd.421)

[6] [https://www.researchgate.net](https://www.researchgate.net/publication/220746940_Event_Models_for_Historical_Perspectives_Determining_Relations_between_High_and_Low_Level_Events_in_Text_Based_on_the_Classification_of_Time_Location_and_Participants)

[7] [https://www.researchgate.net](https://www.researchgate.net/publication/354774191_Visually_Connecting_Historical_Figures_Through_Event_Knowledge_Graphs)

[8] [https://ceur-ws.org](https://ceur-ws.org/Vol-1485/paper3.pdf)

[9] [https://www.mdpi.com](https://www.mdpi.com/2076-3417/11/10/4378)

[10] [https://link.springer.com](https://link.springer.com/article/10.1007/s11042-021-11030-8)

[11] [https://pmc.ncbi.nlm.nih.gov](https://pmc.ncbi.nlm.nih.gov/articles/PMC12067229/)

[12] [https://www.researchgate.net](https://www.researchgate.net/publication/4175573_Visualizing_Historical_Data_Using_Spectrographs)

[13] [https://link.springer.com](https://link.springer.com/chapter/10.1007/978-3-031-18158-0_1)

[14] [https://milvus.io](https://milvus.io/ai-quick-reference/how-does-a-knowledge-graph-differ-from-a-traditional-database)






============================================







To build a narrative knowledge graph for a roguelike like _[Caves of Qud](https://www.google.com/search?kgmid=/g/11j4ffwqgg&q=I+am+looking+to+implement+a+narrative+knowledge+graph+system+to+maintain+lore/world-building+information+in+a+RPG+rougelike+game+that+I+am+creating+%28similar+to+Caves+of+Qud%29.+I+want+to+base+this+on+how+scientists+and+researchers+do+this+for+real+historic+accounts+but+still+make+it+easy+to+integrate+into+the+game.+In+the+long+term+I+am+looking+at+procedurally+generating+narrative+accounts+in+a+way+that+verifies+that+events+could+have+ocured+and+don%27t+conflict.)_, you can adopt the Event-Centric Knowledge Graph (EKG) approach used in digital humanities. This model treats "events" as the primary nodes that link people, places, and time, ensuring historical consistency while remaining flexible for procedural generation.

## 1. The Core Model: Event-Centric Architecture

Instead of linking a character directly to a location, every connection passes through an Event Node. This mirrors the CIDOC CRM standard used by researchers to model complex historical accounts. [1, 2]

- Nodes:
    
    - Actors: Characters, Factions, Deities.
    - Places: Regions, Dungeons, Cities.
    - Events: Battles, Coronations, Plagues, Discoveries.
    - Concepts: Religions, Laws, Mythical Artifacts.
    
- Edges (Relationships):
    
    - `Actor` — _participated_in_ → `Event`
    - `Event` — _occurred_at_ → `Place`
    - `Event` — _occurred_during_ → `Time_Span`
    - `Event` — _caused_ → `Event` (This creates the "narrative chain"). [3]
    

## 2. Implementation for Game Integration

For a roguelike, a full triple-store database (like Neo4j) might be overkill. You can implement a Subjective/Objective Graph system within your game engine: [4]

- Objective Graph: The "ground truth" of what actually happened in your world’s history.
- Subjective Fragments: Small sub-graphs or "lore fragments" given to the player through item descriptions or NPC dialogue. These can be intentionally "fuzzy" or biased versions of the Objective Graph.
- Graph Pattern Matching: Use simple rules to trigger game content. For example: _If (Actor A) participated in (Event B) and (Event B) occurred at (Location C), then (Actor A) has a "Resident" tag for Location C._. [4, 5, 6]

## 3. Procedural Verification & Conflict Resolution

To ensure your generated history "makes sense" and doesn't conflict, use Constraint-Based Generation: [7, 8]

1. Pre-condition Checking: Before adding a new event (e.g., a "Siege"), the system queries the graph: _Does the Faction still exist? Is the City already destroyed?_
2. Temporal Intervals: Use Allen’s Interval Algebra (e.g., _Event A_ must happen _before_ or _during_ _Event B_) to keep your timeline linear and logical.
3. State Tracking: Each event node can store a "World State Change." If a King dies in _Event A_, the graph updates the _Actor_ node's status to "Deceased," preventing them from appearing in _Event B_ fifty years later. [9, 10, 11, 12]

## Comparison for Roguelike Development

|Feature [9, 13]|Entity-Centric (Traditional)|Event-Centric (Researcher Style)|
|---|---|---|
|Lore Focus|What is this thing?|What happened here?|
|Conflict Detection|Difficult (manual flags)|Built-in (causal links between nodes)|
|Narrative Flow|Static descriptions|Dynamic "storylines" and chains|
|Best For|Static RPGs|Emergent History (Dwarf Fortress, Qud)|

Would you like a sample JSON schema for a historical event node to see how these relationships look in code?

  

[1] [https://www.mdpi.com](https://www.mdpi.com/2076-3417/15/22/12063#:~:text=The%20artifact%20knowledge%20graph%20built%20based%20on,with%20few%20studies%20demonstrating%20practical%20implementation%20and%E2%80%94validation.)

[2] [https://www.mdpi.com](https://www.mdpi.com/1999-5903/13/11/277)

[3] [https://aclanthology.org](https://aclanthology.org/2025.acl-long.830.pdf)

[4] [https://www.youtube.com](https://www.youtube.com/watch?v=0aiXBfH6dc8&t=235)

[5] [https://www.youtube.com](https://www.youtube.com/watch?v=-u0SCvkMPVs)

[6] [https://www.youtube.com](https://www.youtube.com/watch?v=-NKKIDYU0sU&t=19)

[7] [https://arxiv.org](https://arxiv.org/html/2505.24803v2)

[8] [https://www.researchgate.net](https://www.researchgate.net/publication/288320122_Designing_procedurally_generated_levels)

[9] [https://pubmed.ncbi.nlm.nih.gov](https://pubmed.ncbi.nlm.nih.gov/37128597/)

[10] [https://arxiv.org](https://arxiv.org/pdf/2311.01928#:~:text=Knowledge%20graphs%20that%20are%20used%20to%20play,model%20like%20a%20recurrent%20neural%20network%20%5B20%5D.)

[11] [https://www.youtube.com](https://www.youtube.com/watch?v=wPb-eZJL9v4&t=95)

[12] [https://ualresearchonline.arts.ac.uk](https://ualresearchonline.arts.ac.uk/id/eprint/25403/1/Guiding%20Generative%20Storytelling%20with%20Knowledge%20Graphs.pdf)

[13] [https://link.springer.com](https://link.springer.com/chapter/10.1007/978-3-031-11609-4_38)



=================================


To organize and extract information from real historical accounts, researchers use formal frameworks that prioritize the relationship between Actors, Places, and Time, specifically to handle the gaps and contradictions typical of the past.

## 1. The Core Research Model: CIDOC CRM

The gold standard for researchers is the [CIDOC Conceptual Reference Model (CRM)](https://cidoc-crm.org/sites/default/files/Documenting%20Events%20in%20Metadata.pdf). Instead of a flat database of "facts," it uses an Event-Centric structure. This is scientific because it doesn't just link a King to a Throne; it links them through a "Coronation Event," which has its own metadata (date, participants, reliability of source). [1]

- Primary Entities:
    
    - E21 Person: The "who".
    - E53 Place: The "where".
    - E2 Temporal Relation: The "when".
    - E5 Event: The core unit that links the three above.
    
- The Benefit: This allows researchers to track conflicting accounts. If two different manuscripts describe the same battle differently, they aren't "errors" in the database; they are two separate "Information Objects" linked to the same "Event" node. [1, 2, 3]

## 2. Methodologies for Extraction

Historians use specific scientific workflows to turn messy text into a usable graph:

- Prosopography (Collective Biography): Researchers extract metadata from thousands of individuals (birth dates, education, social rank) to find patterns in how groups moved or revolted.
- Historical Network Analysis (HNA): This maps "explicit networks" like correspondence (who wrote to whom) or shipping routes. Researchers then use Epistemic Network Analysis (ENA) to visualize how concepts or political ideas spread through these nodes over time.
- Named Entity Recognition (NER): Using tools like Prodigy or custom [Deep Learning](https://pmc.ncbi.nlm.nih.gov/articles/PMC8321201/) models, researchers automatically pull people, dates, and locations from unstructured archives to populate their Knowledge Graphs. [4, 5, 6, 7, 8]

## 3. Verification & Conflict Management

To ensure events could have occurred without conflict, researchers apply:

- Temporal Interval Algebra: Based on Allen's logic, this ensures that if _Event A_ (a character's birth) has a date range, _Event B_ (their death) cannot logically occur before that range starts.
- Spatio-Temporal Analysis: This uses metadata to verify if an actor could have physically traveled between two events in the time provided, often using Linked Open Data (like GeoSPARQL) to calculate travel distances across historical maps. [7, 9]

## Summary of Research Tools

|Tool Type [10, 11, 12, 13]|Purpose|Examples|
|---|---|---|
|Reference Managers|Organize sources and extract basic metadata|Zotero, Mendeley|
|Archival Management|Handle high volumes of primary source photos|Tropy|
|Analysis Environments|Scripting for text mining and graph visualization|[Jupyter Notebooks](https://jupyter.org/), [Programming Historian](https://programminghistorian.org/)|
|Graph Visualization|Map relationships and network lineages|Gephi, Mermaid|

Would you like a deeper look at the Allen's Interval Algebra constraints for managing your timeline?

  

[1] [https://cidoc-crm.org](https://cidoc-crm.org/sites/default/files/Documenting%20Events%20in%20Metadata.pdf)

[2] [https://isprs-archives.copernicus.org](https://isprs-archives.copernicus.org/articles/XLVIII-M-2-2023/943/2023/isprs-archives-XLVIII-M-2-2023-943-2023.pdf)

[3] [https://cidoc.mini.icom.museum](https://cidoc.mini.icom.museum/wp-content/uploads/sites/6/2018/12/37_papers.pdf)

[4] [https://docs.iza.org](https://docs.iza.org/dp13788.pdf)

[5] [https://pmc.ncbi.nlm.nih.gov](https://pmc.ncbi.nlm.nih.gov/articles/PMC5426307/)

[6] [https://pmc.ncbi.nlm.nih.gov](https://pmc.ncbi.nlm.nih.gov/articles/PMC5426307/)

[7] [https://www.sciencedirect.com](https://www.sciencedirect.com/science/article/pii/S2405844022019983)

[8] [https://pmc.ncbi.nlm.nih.gov](https://pmc.ncbi.nlm.nih.gov/articles/PMC8321201/)

[9] [https://gauravkantgoel.medium.com](https://gauravkantgoel.medium.com/data-modeling-patterns-for-historical-data-641f9087e887)

[10] [https://doinghistoryinpublic.org](https://doinghistoryinpublic.org/2025/08/12/top-3-digital-tools-for-doing-history/)

[11] [https://blog.royalhistsoc.org](https://blog.royalhistsoc.org/2022/12/15/historical-researchpart-2-tools-for-the-trade/)

[12] [https://www.youtube.com](https://www.youtube.com/watch?v=ywHSoIXvoIw&t=18)

[13] [https://www.researchgate.net](https://www.researchgate.net/post/What_digital_tools_do_you_find_most_helpful_in_historical_research_and_archiving)
