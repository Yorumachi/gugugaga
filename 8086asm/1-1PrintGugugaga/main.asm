.model small
.stack 100h
.data
        msg db 'gugugaga!!!$'
        count dw 100
.code
main proc
        mov ax,@data
        mov ds,ax
        mov cx,count
print:
        mov ah,09h
        mov dx,offset msg
        int 21h

        mov ah,02h
        mov dl,0Dh
        int 21h

        mov dl,0Ah
        int 21h

        loop print

        ;here is the end of loop=w=
        mov ah,4ch
        mov al,0
        int 21h
main endp
end main

