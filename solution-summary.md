# Solution Summary for Train Connections

## Problem Definition:

* In this assignment I need to find the shortest path between different train stations using different cost functions.
* The link between two stations is supposed to be the shortest.
* The first algorithm that came to my mind is the *Djkstra's Shortest Path*
* I worked on a data set of Indian Railways provided to me.

---

+ Now I will go into details of my implementation and how I approached the problem for each variant of the cost
  functions.

+ To Start the project's guide helped me with understanding the problem a bit more than usual.

## Different cost Functions and the approaches.

First to go on any further in this assignment we need a proper graph to traverse through in order to find the shortest
path
using any algorithm. To build the graph a basic intuition has already been given i.e. first we need to parse the data
and create
station objects which are nodes of the graph and edges will the cost function that could change according to the graph
and the cost
function type that I want to use on the graph. Now I will elaborate on the different graphs that i build using different
cost functions.

### Cost function 1 *Stops*:

This is the first problem that was given to solve. here the cost function will be the stops taken to get to one
particular station i.e. How many
stops it took for me to reach a station *Munich* from Station *Düsseldorf*. For this approach I build the graph with the
stations as the nodes and
the edges are the stations that come next to a particular station i.e. If current station is *Munich* and next station
is *Erlangen* then node will be
*Munich* and edge will be the node *Erlangen*. Edge also have cost here it will be '1'.

### Cost function 2 *Travel time*:

In this problem the Graph remains the same from the first problem but the edges cost will be the travel time that it
takes to reach from one station
to the other. The time taken to reach *Düsseldorf* from *Munich* here as an example I will say suppose 2 *hours* then
the edge cost will be *2x60x60*.

### Cost function 3 *Ticket price*:
Here the cost function is based on two different tickets one is the stop ticket and the other is the train ticket, the
train ticket costs 10 which can be used to travel the entire train as long as we want but the stop ticket is only for one
single stop no matter what the train is.

Here you cannot work with the first graph anymore because the cost function is changed after certain number of stations
i.e. consider this route *Munich* -> *Frankfurt* -> *Berlin* if munich and frankfurt are the 8th and 9th stops after
the first stops then you can travel there with 8 and 9 single tickets respectively, but after frankfurt if you want to 
travel to berlin you don't need stop tickets but the train ticket that is of a particular train that sosts 10 which will
take you to the final destination. 

In this I built a bigger graph that contains more nodes, than the original. How did the graph has more nodes than the 
original ? . If we compare the first graph, each node has only the successor stations as the child node for the above problem
but here the children are not just the successor stations but also as far as stations depending on the cost and how
far the station is from the parent.

### Cost function 4 *Arrival Time*:
Only for the 3rd problem the graph is bigger, but for this problem the graph is smaller the only difference is it also has
additional nodes to know what is the past station compared to a particular node. Since the graph will not change that much
the main focus is on the *Djkstra's algorithm* which is giving me the right solutions. 


-- Warning If you look at the code you may gain memory loss or similar kind of side effects that will effect your day to 
day life. *Sarcasm*