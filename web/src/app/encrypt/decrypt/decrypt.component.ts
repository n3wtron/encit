import { Component, OnInit } from '@angular/core';
import { Identity } from '../../identities/identity.service';
import * as FileSaver from 'file-saver';
import { EncItMessage, EncryptService } from '../encrypt.service';
import { Clipboard } from '@angular/cdk/clipboard';
import { MatSnackBar } from '@angular/material/snack-bar';
import { ErrorService } from '../../common/error.service';

@Component({
  selector: 'app-decrypt',
  templateUrl: './decrypt.component.html',
  styleUrls: ['./decrypt.component.scss'],
})
export class DecryptComponent implements OnInit {
  identity: Identity | undefined;
  message: string | undefined;
  b64File: string | undefined;
  result: EncItMessage | undefined;

  constructor(
    private readonly encryptService: EncryptService,
    private readonly errorService: ErrorService,
    private readonly clipboard: Clipboard,
    private readonly _snackBar: MatSnackBar
  ) {}

  ngOnInit(): void {}

  decryptFile() {
    if (!!this.b64File) {
      this.encryptService.decrypt(this.b64File, this.identity?.name).subscribe(
        (result) => {
          this.result = result;
        },
        (error) => this.errorService.error(error)
      );
    }
  }

  decryptMessage(download?: boolean) {
    if (!!this.message) {
      this.encryptService.decrypt(this.message, this.identity?.name).subscribe(
        (result) => {
          this.result = result;
        },
        (error) => this.errorService.error(error)
      );
    }
  }

  uploadFile(fileUploadEvent: EventTarget | null) {
    if (
      fileUploadEvent &&
      fileUploadEvent instanceof HTMLInputElement &&
      fileUploadEvent.files &&
      fileUploadEvent.files.length == 1
    ) {
      let fl = fileUploadEvent.files[0];
      let fileReader = new FileReader();
      fileReader.readAsBinaryString(fl);
      fileReader.onload = (event) => {
        this.b64File = event!.target!.result!.toString();
      };
    }
  }

  download() {
    if (this.result && this.result.payload) {
      let strBytes = atob(this.result.payload);
      let bytes = new Uint8Array(strBytes.length);
      for (let i = 0; i < strBytes.length; i++) {
        bytes[i] = strBytes.charCodeAt(i);
      }
      let blob = new Blob([bytes]);
      FileSaver.saveAs(blob, this.result.subject);
    }
  }

  reset() {
    this.result = undefined;
    this.message = '';
    this.b64File = undefined;
  }
}
