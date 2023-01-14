// Include the most common headers from the C standard library


// Include the main libnx system header, for Switch development
#include <switch.h>
#include "bindings.h"

enum State {
    Year,
    Day,
    Run,
    End
};

void reprint(int year, int day, const State &state) {
    consoleClear();

    switch (state) {
        case Year:
            printf("Select Year: %d", year);
            break;
        case Day:
            printf("Select Day: %d", day);
            break;
        default:
            break;
    }
}

int main(int argc, char* argv[])
{
    consoleInit(NULL);

    init_heap();

    padConfigureInput(1, HidNpadStyleSet_NpadStandard);

    PadState pad;
    padInitializeAny(&pad);

    int year = 2022;
    int day = 1;
    State state = Year;

    reprint(year, day, state);
    while(appletMainLoop())
    {
        if (state == Run)
        {
            char fileName[40];

            snprintf(fileName, 40, "sdmc:/switch/advent_inputs/%d_%d.txt", year, day);

            FILE *fptr;

            if((fptr = fopen(fileName, "r")) == NULL)
            {
                printf("Unable to find or open the input file for %d day %d.\n\nSpecified path was %s", year, day, fileName);
            }
            else
            {
                long fileLen;

                fseek(fptr, 0L, SEEK_END);
                fileLen = ftell(fptr);
                fseek(fptr, 0L, SEEK_SET);

                if (fileLen == 0)
                {
                    puts("The input is empty!");
                    fclose(fptr);
                }
                else
                {
                    char *input = (char*)calloc(fileLen, sizeof(char));

                    if (input == NULL)
                    {
                        printf("Unable to allocate memory for input");
                        fclose(fptr);
                    }
                    else
                    {
                        fread(input, sizeof(char), fileLen, fptr);
                        fclose(fptr);

                        char* result = run_day(year, day, input);
                        puts(result);
                        free_result(result);
                        free(input);
                    }
                }
            }
            puts("\n\nPress + to exit or - to return to the starting menu.");
            state = End;
        }
        // Scan the gamepad. This should be done once for each frame
        padUpdate(&pad);

        // padGetButtonsDown returns the set of buttons that have been newly pressed in this frame compared to the previous one
        u64 kDown = padGetButtonsDown(&pad);

        if (kDown & HidNpadButton_Plus) break; // break in order to return to hbmenu

        if (kDown & HidNpadButton_Down)
        {
            switch (state)
            {
                case Year:
                    // Todo: Would select the year to execute
                    break;
                case Day:
                    if (day == 1)
                    {
                        day = 25;
                    } else
                    {
                        day -= 1;
                    }
                    break;
                default:
                    break;
            }
            reprint(year, day, state);
        }
        else if (kDown & HidNpadButton_Up)
        {
            switch (state)
            {
                case Year:
                    // Todo: Would select the year to execute
                    break;
                case Day:
                    if (day == 25)
                    {
                        day = 1;
                    }
                    else
                    {
                        day += 1;
                    }
                    break;
                default:
                    break;
            }
            reprint(year, day, state);
        }
        else if (kDown & HidNpadButton_A)
        {
            if (state == Year)
            {
                state = Day;
                reprint(year, day, state);
            }
            else if (state == Day)
            {
                state = Run;
                printf("\n\nRunning %d day %d...\n\n", year, day);
            }
        }
        else if (kDown & HidNpadButton_B)
        {
            if (state == Day)
            {
                state = Year;
                reprint(year, day, state);
            }
        }
        else if ((kDown & HidNpadButton_Minus) && state == End)
        {
            state = Year;
            reprint(year, day, state);
        }

        consoleUpdate(NULL);
    }

    consoleExit(NULL);
    return 0;
}
