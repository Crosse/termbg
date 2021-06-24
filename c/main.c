/*
 * This file includes code that smells a lot like a distillation of
 * https://github.com/JessThrysoee/xtermcontrol, which seems to be GPLv2. Thus,
 * to hedge my bets, I'm making this file also available under the GPLv2 license
 * and not the MIT license that the rest of the repo uses.
 *
 * Copyright (c) 2021 Seth Wright <seth@crosse.org>.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

#include <sys/types.h>
#include <sys/uio.h>

#include <errno.h>
#include <fcntl.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>


int
main(int argc, char** argv)
{
    FILE *fout = fdopen(STDOUT_FILENO, "r+");
    if (fout == NULL) {
        perror("fdopen()");
        return EXIT_FAILURE;
    }

    struct termios tty_ts;
    if (tcgetattr(STDIN_FILENO, &tty_ts) == -1) {
        perror("tcgetattr()");
        return EXIT_FAILURE;
    }

    struct termios orig = tty_ts;
    /* cfmakeraw(&raw); */
    tty_ts.c_iflag = 0;
    tty_ts.c_lflag = 0;
    tty_ts.c_cc[VMIN] = 0;
    tty_ts.c_cc[VTIME] = 1;
    tty_ts.c_lflag &= ~(ICANON | ECHO);

    /* printf("setting terminal to raw mode\n"); */
    if (tcsetattr(STDIN_FILENO, TCSANOW, &tty_ts) == -1) {
        perror("tcsetattr()");
        goto done;
    }

    const char magic[] = "\033]11;?\033\\";
    const char tmux_fmt[] = "\033Ptmux;\033%s\033\\";
    const char *fmt = "%s";
    char s[BUFSIZ] = { 0 };

    if (getenv("TMUX")) {
        fmt = tmux_fmt;
    }

    if (snprintf(s, BUFSIZ, fmt, magic) < 0) {
      perror("asprintf()");
      goto done;
    }

    if (write(STDOUT_FILENO, magic, strlen(magic)) == -1) {
        perror("write()");
        goto done;
    }
    /* fflush(fout); */

    char buf[128] = { 0 };
    int len = 0;
    do {
        char c = 0;
        ssize_t res = read(STDIN_FILENO, &c, 1);
        if (res == 0 || (res == -1 && errno == EAGAIN)) {
              break;
        }
        if (c == '\007') {
            break;
        }
        buf[len] = c;
        len += res;
    } while (len < sizeof(buf));

done:
    /* printf("setting terminal to cooked mode\r\n"); */
    if (tcsetattr(STDIN_FILENO, TCSANOW, (const struct termios *)&orig) == -1) {
        perror("tcsetattr()");
        return EXIT_FAILURE;
    }

    if (strnlen(buf, sizeof(buf)) == 0) {
        printf("no response from the terminal\n");
        return EXIT_FAILURE;
    }

    char *p = strchr(buf, ';');
    if (p == NULL) {
      p = buf;
    } else {
      p++;
    }
    printf("%s\n", p);

    if ((p = strchr(p, ':')) == NULL) {
        printf("format seems wrong");
        return EXIT_SUCCESS;
    }
    p++;

    if (p - buf + len < 11) {
        printf("leftover isn't large enough\n");
        return EXIT_SUCCESS;
    }

    uint32_t r = strtol(p, &p, 16) >> 8;
    uint32_t g = strtol(p+1, &p, 16) >> 8;
    uint32_t b = strtol(p+1, &p, 16) >> 8;

    printf("r: %2x, g: %2x, b: %2x\n", r, g, b);
    printf("r: %2d, g: %2d, b: %2d\n", r, g, b);

    double hsp = sqrt(0.299 * (r * r) + 0.587 * (g * g) + 0.114 * (b * b));
    printf("HSP: %0.2f\n", hsp);
    if (hsp > 127.5) {
        printf("this seems like a light color\n");
    } else {
        printf("this seems like a dark color\n");
    }
    return EXIT_SUCCESS;
}
