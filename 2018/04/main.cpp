#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <ctime>

using namespace std;

enum EVENT_TYPE { AWOKE, ASLEEP, SHIFT };

class Event {
	public:
	string description;
	tm *datetime;
	EVENT_TYPE type;

	Event(string line) {
		datetime = new tm();

		strptime(line.c_str(), "[%Y-%m-%d %H:%M", datetime);
		description = line.substr(line.find("] ") + 2);

		if (description.find("begins shift") != std::string::npos) {
			type = SHIFT;
		} else if (line.find("falls asleep") != std::string::npos) {
			type = ASLEEP;
		} else if (line.find("wakes up") != std::string::npos) {
			type = AWOKE;
		}
	}

	int guard_id() {
		if (type != SHIFT) return 0;
		int id;
		sscanf(description.c_str(), "Guard #%d", &id);
		return id;
	}

	string shift_date(string format = "%Y-%m-%d") {
		tm date = *datetime;
		if (date.tm_hour == 23) date.tm_mday += 1;
		char buff[100];
		strftime(buff, 100, format.c_str(), &date);
		string str(buff);
		return str;
	}
};

class Shift {
	public:
	string datestr;
	int guard_id;
	tm *date;
	vector<Event> events;

	Shift(string datestring, int id) {
		datestr = datestring;
		date = new tm();
		guard_id = id;
		strptime(datestring.c_str(), "%Y-%m-%d", date);
	}

	vector<int> minutes_slept() {
		vector<int> total;
		vector<Event> asleep_events;

		for (auto event : events) {
			if (event.type != ASLEEP) continue;
			asleep_events.push_back(event);
		}

		for (auto asleep_event : asleep_events) {
			auto awoke_event = *(find_if(
				events.begin(), events.end(),
				[&](Event &event) {
					return event.type == AWOKE && event.datetime->tm_min > asleep_event.datetime->tm_min;
				}
			));

			for (int i = asleep_event.datetime->tm_min; i < awoke_event.datetime->tm_min; i++) {
				total.push_back(i);
			}
		}

		return total;
	}
};

class Guard {
	public:
	int id;
	vector<vector<int>> sleep_periods;

	Guard(int guard_id) {
		id = guard_id;
	}

	int total_slept() {
		int total = 0;
		for (auto period : sleep_periods) total += period.size();
		return total;
	}

	int most_slept_min(int *occurances = NULL) {
		int minute = 0, max_occurances = 0, c;

		for (int i = 0; i < 59; i++) {
			c = 0;

			for (auto period : sleep_periods) {
				if (find(period.begin(), period.end(), i) != period.end()) c++;
			}

			if (c > max_occurances) {
				max_occurances = c;
				minute = i;
			}
		}

		*occurances = max_occurances;
		return minute;
	}
};

int main() {
	// Part 1
	ifstream input;
	input.open("input.txt");

	string line;
	vector<Event> events;
	while (getline(input, line)) events.push_back(*(new Event(line)));

	sort(
		events.begin(), events.end(),
		[](Event &e1, Event &e2) {
			return e1.shift_date("%Y-%m-%dT%H:%M") < e2.shift_date("%Y-%m-%dT%H:%M");
		}
	);

	vector<string> dates;
	for (auto event : events) {
		string date = event.shift_date();
		if (find(dates.begin(), dates.end(), date) != dates.end()) continue;
		dates.push_back(date);
	}

	vector<Shift> shifts;
	vector<int> guard_ids;
	for (auto date: dates) {
		auto shift_event = *(find_if(
			events.begin(), events.end(),
			[&](Event &event) { return event.type == SHIFT && event.shift_date() == date; }
		));

		int guard_id = shift_event.guard_id();
		auto *shift = new Shift(date, guard_id);

		for (auto &event : events) {
			if (event.type == SHIFT) continue;
			if (event.shift_date() != date) continue;
			shift->events.push_back(event);
		}

		if (find(guard_ids.begin(), guard_ids.end(), guard_id) == guard_ids.end()) {
			guard_ids.push_back(guard_id);
		}
		shifts.push_back(*shift);
	}

	vector<Guard> guards;
	for (auto id : guard_ids) guards.push_back(*(new Guard(id)));

	int max_slept = 0;
	int guard_id;

	for (auto shift : shifts) {
		auto guard = find_if(
			guards.begin(), guards.end(),
			[&](Guard &guard) { return guard.id == shift.guard_id; }
		);

		guard->sleep_periods.push_back(shift.minutes_slept());
		if (guard->total_slept() > max_slept) {
			max_slept = guard->total_slept();
			guard_id = guard->id;
		}
	}

	Guard guard = *(find_if(
			guards.begin(), guards.end(),
			[&](Guard &guard) { return guard.id == guard_id; }
	));

	int c;
	cout << "Part 1: " << guard.id * guard.most_slept_min(&c) << endl;

	// Part 2
	int minute, max_occurances = 0;
	for (auto guard : guards) {
		int m = guard.most_slept_min(&c);
		if (c > max_occurances) {
			minute = m;
			max_occurances = c;
			guard_id = guard.id;
		}
	}

	cout << "Part 2: " << guard_id * minute << endl;
	input.close();
	return 0;
}
