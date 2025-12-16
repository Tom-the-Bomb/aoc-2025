"""
Day 9: Movie Theater

https://adventofcode.com/2025/day/9
"""
__all__ = ('Day9',)

from typing import ClassVar
from itertools import combinations

from ..solution import Solution

class Day9(Solution):
    NAME: ClassVar[str] = 'Movie Theater'

    def _parse_input(self, inp: str) -> list[tuple[int, int]]:
        return [
            (int((a := raw.split(','))[0]), int(a[1]))
            for raw in inp.splitlines()
        ]

    def _point_in_polygon(self, x: int, y: int, polygon: list[tuple[int, int]]) -> bool:
        """Ray-casting algorithm to determine if a point is in a polygon."""
        inside = False
        n_points = len(polygon)

        for i in range(n_points):
            ax, ay = polygon[i]

            if i == n_points - 1:
                # wrap around to first point
                bx, by = polygon[0]
            else:
                bx, by = polygon[i + 1]

            # [1] `ax == bx` => vertical edge
            #
            # [2] check if `y` is between `ay` and `by`
            # and handle the edge case where `y` is exactly `ay` or `by`
            # we need to establish a standard to include only 1 of the endpoints
            #
            # => `min(ay, by) <= y < max(ay, by)` (count lower endpoint, not upper)
            # OR
            # => `min(ay, by) < y <= max(ay, by)` (count upper endpoint, not lower)
            #
            # the reason we do this is for cases like:
            # |.  _| ____ (<--ray)
            # |  |
            # where the point is inside the polygon, but flipping on both vertical edges would result in `inside = False`
            # therefore if we only include i.e. the intersection with the lower endpoint
            # and exclude the intersection with the upper endpoint, we count this point as inside
            #
            # likewise (inside):
            # |. |_   ___ (<--ray)
            # |    |
            #
            # but we can rewrite this to be more efficient instead of determining min/maxes
            # let's write this for `min(ay, by) <= y < max(ay, by)`
            #
            # ay > y          | by > y          | (ay > y) != (by > y)
            # ----------------|-----------------|---------------------
            # True  (ay > y)  | True  (by > y)  | False (`ay` and `by` both above `y`)
            # False (ay <= y) | False (by <= y) | False (`ay` and `by` both below or equal to `y`)
            # True  (ay > y)  | False (by <= y) | True  (by <= y < ay) where ay < by
            # False (ay <= y) | True  (by > y)  | True  (ay <= y < by) where by < ay
            #
            # [3] check if the x-coordinate of the point is to the left of
            # where the edge intersects the casted ray since we're raycasting from the right (leftwards)
            #
            # NOTE: this is simply the x-coordinate of the vertical edge itself
            # hence `x < ax`, since our polygon is made of only horizontal and vertical edges
            # if not, we must develop an expression for the x-coordinate of the intersection point
            # and use that instead of `ax`
            if ax == bx and (ay > y) != (by > y) and x < ax:
                inside = not inside
        return inside

    def _intersects(
        self,
        line1_start: tuple[int, int],
        line1_end: tuple[int, int],
        line2_start: tuple[int, int],
        line2_end: tuple[int, int],
    ) -> bool:
        """Returns whether or not 2 lines intersect. Lines must be either vertical or horizontal.

        Edge cases where lines are collinear, or touch at endpoints ('T' / 'L' intersections) are considered non-intersecting.
        """
        if (line1_start[0] == line1_end[0]) == (line2_start[0] == line2_end[0]):
            # Both lines are vertical or both lines are horizontal
            return False

        start1, end1 = sorted((line1_start, line1_end))
        start2, end2 = sorted((line2_start, line2_end))

        # line1 is vertical, line2 is horizontal
        if start1[0] == end1[0]:
            return (
                # x-coordinate of vertical line is between endpoints of horizontal line
                start2[0] < start1[0] < end2[0] and
                # y-coordinate of horizontal line is between endpoints of vertical line
                start1[1] < start2[1] < end1[1]
            )
        # line1 is horizontal, line2 is vertical
        else:
            return (
                # x-coordinate of horizontal line is between endpoints of vertical line
                start1[0] < start2[0] < end1[0] and
                # y-coordinate of vertical line is between endpoints of horizontal line
                start2[1] < start1[1] < end2[1]
            )

    def part_one(self, inp: str) -> int:
        return max(
            # +1 since `x1=1` and `x2=3`gives '###' but `x2 - x1 = 3 - 1 = 2`, but we want a side-length of 3
            (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)
            for (x1, y1), (x2, y2) in combinations(self._parse_input(inp), 2)
        )

    def part_two(self, inp: str) -> int:
        points = self._parse_input(inp)
        max_area = 0

        n_points = len(points)

        for (x1, y1), (x2, y2) in combinations(points, 2):
            # optimization: check if the area of our rectangle is even larger than current max_area
            # before we perform expensive checks to check of the rectangle is valid
            area = (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)
            if area <= max_area:
                continue

            # the trick of the problem is to assume that rectangles of maximum area will have dimensions `>= 2`
            # (i.e. lines are non-valid)
            #
            # therefore, we are able to check if the rectangle that is 1 unit smaller on all sides is valid
            # because if that rectangle is valid, then the larger rectangle must also be valid
            #
            # this allows us to avoid edge-cases where rectangle edges touch polygon edges at endpoints
            # since those cases are difficult to handle, as we don't know which direction the polygon edge goes from that point
            #
            # now we are able to consider the smaller rectangle, and check if its strictly inside (not touching polygon edges at all => easy!)
            # and then assume this implies our original rectangle is valid as well (can touch polygon edges)
            #
            # since the `+y` direction is downwards, we `+1` to the minmium y-coord to minimize the rectangle
            # and we `-1` to the minimize the x-coord
            rect_corners = [
                (min(x1, x2) + 1, min(y1, y2) + 1), # top-left
                (min(x1, x2) + 1, max(y1, y2) - 1), # bottom-left
                (max(x1, x2) - 1, max(y1, y2) - 1), # bottom-right
                (max(x1, x2) - 1, min(y1, y2) + 1), # top-right
            ]

            # Check if all rectangle corners are inside the polygon (not on edge)
            if not all(
                self._point_in_polygon(px, py, points)
                for px, py in rect_corners
            ):
                continue

            # check if any rectangle edge intersects with any polygon edge (with the criteria defined above)
            intersects = False
            # check all edges of the rectangle
            for i in range(4):
                edge_start = rect_corners[i]

                if i == 3:
                    # wrap around to first corner
                    edge_end = rect_corners[0]
                else:
                    edge_end = rect_corners[i + 1]

                # check all edges of the polygon
                for j in range(n_points):
                    line_start = points[j]

                    if j == n_points - 1:
                        # wrap around to first point
                        line_end = points[0]
                    else:
                        line_end = points[j + 1]

                    if self._intersects(edge_start, edge_end, line_start, line_end):
                        intersects = True
                        break
                # break out of outer-loop early too if we know it intersects
                if intersects:
                    break

            if intersects:
                continue

            max_area = area
        return max_area

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 4777967538
        assert p2 == 1439894345