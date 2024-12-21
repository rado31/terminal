import type { MonthButton } from '$lib/types/datepicker'

export class Datepicker {
	public input: Date = $state(new Date())
	public currDate: Date = $state(new Date())
	public isOpen: boolean = $state(false)
	public day: number = $state(0)
	public month: number = $state(0)
	public year: number = $state(0)
	public prevMonthButtons: MonthButton[] = $state([])
	public currMonthButtons: MonthButton[] = $state([])
	public nextMonthButtons: MonthButton[] = $state([])

	constructor() {
		this.initOrReset()
	}

	public initOrReset() {
		const date = new Date()

		this.input = date
		this.currDate = date

		this.day = this.input.getDate()
		this.month = this.input.getMonth()
		this.year = this.input.getFullYear()
		this.isOpen = false

		this.resetButtons()
		this.initPrevMonthButtons()
		this.initCurrMonthButtons()
		this.initNextMonthButtons()
	}

	public prevMonth() {
		this.month -= 1

		if (this.month === -1) {
			this.year -= 1
			this.month = 11
		}

		this.resetButtons()
		this.initPrevMonthButtons()
		this.initCurrMonthButtons()
		this.initNextMonthButtons()
	}

	public nextMonth() {
		this.month += 1

		if (this.month === 12) {
			this.year += 1
			this.month = 0
		}

		this.resetButtons()
		this.initPrevMonthButtons()
		this.initCurrMonthButtons()
		this.initNextMonthButtons()
	}

	private resetButtons() {
		this.prevMonthButtons = []
		this.currMonthButtons = []
		this.nextMonthButtons = []
	}

	private initPrevMonthButtons() {
		// last day of prev month
		const lastDay = new Date(this.year, this.month, 0)

		for (let i = 1; i <= lastDay.getDay(); i++) {
			const text = lastDay.getDate() - lastDay.getDay() + i
			const date = new Date(this.year, this.month - 1, text)
			this.prevMonthButtons.push({ text, date })
		}
	}

	private initCurrMonthButtons() {
		// last day of current month
		const lastDay = new Date(this.year, this.month + 1, 0)

		for (let i = 1; i <= lastDay.getDate(); i++) {
			const date = new Date(this.year, this.month, i)
			this.currMonthButtons.push({ text: i, date })
		}
	}

	private initNextMonthButtons() {
		// first day of next month
		const firstDay = new Date(this.year, this.month + 1, 1)

		for (let i = firstDay.getDay(); i <= 7; i++) {
			const text = firstDay.getDate() - firstDay.getDay() + i
			const date = new Date(this.year, this.month + 1, text)
			this.nextMonthButtons.push({ text, date })
		}
	}

	public getMonthTitleByID() {
		switch (this.month) {
			case 0:
				return 'Ýanwar'
			case 1:
				return 'Fewral'
			case 2:
				return 'Mart'
			case 3:
				return 'Aprel'
			case 4:
				return 'Maý'
			case 5:
				return 'Iýun'
			case 6:
				return 'Iýul'
			case 7:
				return 'Awgust'
			case 8:
				return 'Sentýabr'
			case 9:
				return 'Oktýabr'
			case 10:
				return 'Noýabr'
			case 11:
				return 'Dekabr'
		}
	}

	public formatedDateInput() {
		return (
			this.input.getFullYear() +
			'-' +
			(this.input.getMonth() + 1) +
			'-' +
			this.input.getDate()
		)
	}
}
