import logging

from playsound import playsound

WARNING_SOUND_PATH = './sounds/warning.wav'
CRITICAL_SOUND_PATH = './sounds/critical.mp3'


def play_warning_sound() -> None:
    playsound(WARNING_SOUND_PATH)


def play_critical_sound() -> None:
    playsound(CRITICAL_SOUND_PATH)


def test_sounds() -> None:
    play_warning_sound()
    play_critical_sound()


if __name__ == '__main__':
    logging.basicConfig(level=logging.DEBUG)
    test_sounds()
