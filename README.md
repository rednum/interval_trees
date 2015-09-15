# Introduction
This is a variant of interval tree data structure, containg two types of this structure - segment-point tree and a point-segment tree. The segment-point tree allows to insert segments and query about points, whereas the point-segment tree allow to insert points and query about segments. All operations run in O(log(tree size)) complexity. The trees are parametrisable by operation you want to support, and you can use them to solve problems like:
- what is the sum of segments in some points?
- what is the product of segments in some points?
- what is the sum of points in some segment?
- what is the biggest point in some segment?
- what is the smallest point in some segment?
Note that this allows for solving "online" version of problems - ie. modification of segment set and queries can be mixed.


# Usage

Please refer to examples/... directory.

# Naming

I'm not sure what is the correct name for this data structure - I learned it in my algorithm class under polish name "drzewo przedziałowe", which could be translated to something like "interval tree". However, it seems like it slightly differs from standard interval tree implementation - instead of storing all intervals, I keep only relevant data in node. (Though, technically, this data could be set of all segments containg this node).
