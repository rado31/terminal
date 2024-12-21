export class Keyboard {
	public isOpen: boolean = $state(false)
	public input: string = $state('')
	// prettier-ignore
	public keys = [
		'1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'BACKSPACE',
		'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P',
		'-', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'ENTER',
		'SHIFT', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '?',
		'SPACE',
	]

	constructor() {
		this.initOrReset()
	}

	public initOrReset() {
		this.isOpen = false
		this.input = ''
	}

	public toggle() {
		this.isOpen = !this.isOpen
	}
}
