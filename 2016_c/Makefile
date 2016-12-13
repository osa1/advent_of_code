CC:=gcc -std=c99

ALL: point_map_test day_1 day_2 day_2_part_2 \
	day_3 day_3_part_2 day_4 day_5 day_6 day_7 \
	day_8 day_9 day_9_part_2 day_10 day_12 \
	queue_test

point_map.o: src/point_map.c src/point_map.h
	$(CC) $< -c -o $@ -g -Wall

common.o: src/common.c src/common.h
	$(CC) $< -c -o $@ -g -Wall

point_map_test: test/point_map.c point_map.o
	$(CC) $^ -o $@ -g -Wall

day_1: src/day_1.c point_map.o common.o
	$(CC) $^ -o $@ -g -Wall

day_2: src/day_2.c common.o
	$(CC) $^ -o $@ -g -Wall

day_2_part_2: src/day_2_part_2.c common.o
	$(CC) $^ -o $@ -g -Wall

day_3: src/day_3.c
	$(CC) $^ -o $@ -g -Wall

day_3_part_2: src/day_3_part_2.c
	$(CC) $^ -o $@ -g -Wall

day_4: src/day_4.c
	$(CC) $^ -o $@ -g -Wall

day_5: src/day_5.c
	$(CC) $^ -o $@ -g -Wall -lcrypto

day_6: src/day_6.c common.o
	$(CC) $^ -o $@ -g -Wall

day_7: src/day_7.c common.o
	$(CC) $^ -o $@ -g -Wall

day_8: src/day_8.c common.o
	$(CC) $^ -o $@ -g -Wall

day_9: src/day_9.c common.o
	$(CC) $^ -o $@ -g -Wall

day_9_part_2: src/day_9_part_2.c common.o
	$(CC) $^ -o $@ -g -Wall

day_10: src/day_10.c common.o
	$(CC) $^ -o $@ -g -Wall

day_12: src/day_12.c common.o
	$(CC) $^ -o $@ -g -Wall

queue.o: src/queue.c src/queue.h
	$(CC) $< -c -o $@ -g -Wall

queue_test: test/queue.c queue.o common.o
	$(CC) $^ -o $@ -g -Wall

clean:
	rm -f day_1 point_map.o point_map_test common.o \
		day_2 day_2_part_2 day_3 day_3_part_2 day_4 \
		day_5 day_6 day_7 day_8 day_9 day_9_part_2 \
		day_10 day_12 queue.o queue_test
