export interface ButtonProp {
	button: MonthButton
	type?: ButtonType
}

export interface MonthButton {
	text: number
	date: Date
}

export enum ButtonType {
	Prev,
	Next,
}
