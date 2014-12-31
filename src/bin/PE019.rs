fn main() {
	let month_days = [31i,//January
						28,//February
						31,//March
						30,//April
						31,//May
						30,//June
						31,//July
						31,//August
						30,//September
						31,//October
						30,//November
						31];//December
	let mut day = 1i; //Sunday=0, Monday=1, etc.
	//get to start of century
	for num_days in month_days.iter() {
		//note: no leap year since 1900 is not divisible by 400
		day += *num_days;
	}
	day = day % 7; //normalize to day of the week for 1/1/1901
	//println!("{}", day);
	//compute rest of the century
	let mut count = if day == 0 { 1i } else { 0i };
	let mut year = 1901i;
	while year <= 2000 {
		for i in range(0u, 12) {
			let mut num_days:int = month_days[i];
			if (i==1) && (year%4==0) { num_days += 1; } //leap year
			day = (day + num_days) % 7; //get day of week
			if day == 0 {
				count += 1;
				//println!("{}, {}", year, i+1);
			}
		}
		if (year==2000) && (day==0) { count -= 1; } //Just in case - year=2000 here means the end of the loop is 1/1/2001
		year += 1
	}
	println!("{}", count);
}