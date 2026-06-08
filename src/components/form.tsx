import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldSeparator,
  FieldSet,
} from "@/components/ui/field";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Card,
  CardAction,
  CardContent,
  CardHeader,
  CardTitle,
} from "./ui/card";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "./ui/dialog";

export function CharacterForm() {
  const [nickname, setNickname] = useState("");
  const [sex, setSex] = useState("");
  const [race, setRace] = useState("");
  const [language, setLanguage] = useState("");
  const reset = () => {
    setSex("");
    setRace("");
    setLanguage("");
  };
  async function generate() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setNickname(await invoke("generate_nickname", { sex, race, language }));
  }
  return (
    <Dialog>
      <Card className="w-full max-w-xl">
        <form
          className="p-8"
          onSubmit={(e) => {
            e.preventDefault();
            generate();
          }}
        >
          <FieldGroup>
            <FieldSet>
              <CardHeader className="mb-12 uppercase text-center">
                <CardTitle>Параметры никнейма</CardTitle>
              </CardHeader>
              <CardContent>
                {" "}
                <FieldGroup>
                  <div className="grid grid-cols-3 gap-2">
                    <Field>
                      <FieldLabel htmlFor="sex">Пол</FieldLabel>
                      <Select value={sex} onValueChange={setSex}>
                        <SelectTrigger id="sex">
                          <SelectValue placeholder="Выберите пол" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectItem value="male">Мужской</SelectItem>
                            <SelectItem value="female">Женский</SelectItem>
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </Field>
                    <Field>
                      <FieldLabel htmlFor="race">Раса</FieldLabel>
                      <Select value={race} onValueChange={setRace}>
                        <SelectTrigger id="race">
                          <SelectValue placeholder="Выберите расу" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectItem value="elf">Эльф</SelectItem>
                            <SelectItem value="human">Человек</SelectItem>
                            <SelectItem value="orc">Орк</SelectItem>
                            <SelectItem value="demon">Демон</SelectItem>
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </Field>
                    <Field>
                      <FieldLabel htmlFor="language">Язык</FieldLabel>
                      <Select value={language} onValueChange={setLanguage}>
                        <SelectTrigger id="language">
                          <SelectValue placeholder="Выберите язык" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectItem value="ru">Русский</SelectItem>
                            <SelectItem value="en">Английский</SelectItem>
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </Field>
                  </div>
                </FieldGroup>
              </CardContent>
            </FieldSet>
            <FieldSeparator />
            <CardAction>
              <Field orientation="horizontal">
                <DialogTrigger>
                  <Button type="submit">Создать</Button>
                </DialogTrigger>
                <Button variant="outline" type="reset" onClick={reset}>
                  Сбросить
                </Button>
              </Field>
            </CardAction>
          </FieldGroup>
        </form>
        <DialogContent className="sm:max-w-sm">
          <DialogHeader>
            <h2 className="font-heading text-2xl">Ваш никнейм</h2>
          </DialogHeader>
          <DialogDescription>
            {nickname !== "" && (
              <p className="font-bold text-xl font-heading">{nickname}</p>
            )}
          </DialogDescription>
          <DialogFooter>
            <Button onClick={() => navigator.clipboard.writeText(nickname)}>
              Скопировать
            </Button>
            <DialogClose asChild>
              <Button variant="outline">Отмена</Button>
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </Card>
    </Dialog>
  );
}
