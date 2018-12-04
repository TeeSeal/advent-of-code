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

int point_in_claim(struct Point point, struct Claim claim) {
	return
		point.x >= claim.x &&
		point.x < claim.x + claim.width &&
		point.y >= claim.y &&
		point.y < claim.y + claim.height;
}

struct Claim* find_non_overlapping_claim(struct Claim *claims, int claim_count, struct Point *points, int point_count) {
	int found_point;

	for (int i = 0; i < claim_count; i++) {
		found_point = 0;

		for (int j = 0; j < point_count; j++) {
			if (point_in_claim(points[j], claims[i])) {
				found_point = 1;
				break;
			}
		}

		if (!found_point) return &claims[i];
	}

	return NULL;
}

int main() {
	FILE* input;
	input = fopen("input.txt", "r");

	int claim_count = 0;
	struct Claim *claims;

	for (;;claim_count++) {
		claims = realloc(claims, (claim_count + 1) * sizeof(struct Claim));

		if (
			fscanf(
				input, "#%d @ %d,%d: %dx%d\n",
				&claims[claim_count].id,
				&claims[claim_count].x,
				&claims[claim_count].y,
				&claims[claim_count].width,
				&claims[claim_count].height
			) == EOF
		) break;
	}

	int point_count = 0;
	struct Point *points;

	for (int i = 0; i < claim_count - 1; i++) {
		for (int j = i + 1; j < claim_count; j++) {
			overlapping_points(claims[i], claims[j], &points, &point_count);
		}
	}

	printf("Overlapping square inches: %d\n", point_count);

	struct Claim *non_overlapping_claim = find_non_overlapping_claim(claims, claim_count, points, point_count);

	printf("ID of claim that doesn't overlap: %d\n", (*non_overlapping_claim).id);
	return 0;
}
