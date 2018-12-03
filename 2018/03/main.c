#include <stdio.h>
#include <stdlib.h>

struct Claim {
	int id;
	int x;
	int y;
	int width;
	int height;
};

struct Point {
	int x;
	int y;
};

int has_point(int x, int y, struct Point *points, int size) {
	for (int i = 0; i < size; i++) {
		if (points[i].x == x && points[i].y == y) return 1;
	}
	return 0;
}

void overlapping_points(struct Claim c1, struct Claim c2, struct Point **points, int *size) {
	for (int x = c1.x; x < c1.x + c1.width; x++) {
		for (int y = c1.y; y < c1.y + c1.height; y++) {
			if (x >= c2.x && x < c2.x + c2.width && y >= c2.y && y < c2.y + c2.height) {
				if (has_point(x, y, *points, *size)) continue;
				*points = realloc(*points, (*size + 1) * sizeof(struct Point));

				(*points)[*size].x = x;
				(*points)[*size].y = y;
				(*size)++;
			}
		}
	}
}

int main() {
	FILE* input;
	input = fopen("input.txt", "r");

	int k = 0;
	struct Claim *claims;

	for (;;k++) {
		claims = realloc(claims, (k + 1) * sizeof(struct Claim));

		if (
			fscanf(
				input, "#%d @ %d,%d: %dx%d\n",
				&claims[k].id,
				&claims[k].x,
				&claims[k].y,
				&claims[k].width,
				&claims[k].height
			) == EOF
		) break;
	}

	int point_count = 0;
	struct Point *points;

	for (int i = 0; i < k - 1; i++) {
		for (int j = i + 1; j < k; j++) {
			overlapping_points(claims[i], claims[j], &points, &point_count);
		}
	}

	printf("Total: %d\n", point_count);
	return 0;
}
