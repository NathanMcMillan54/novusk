global add

add:
	rep hint_nop55 edx
	lea eax,[edi+esi]
	ret
